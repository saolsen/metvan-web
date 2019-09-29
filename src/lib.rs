// Next Steps

// Are we doing puzzles or are we doing fighting?
// I'm not sure yet.
// better jump code, or abilities, or map design or what?
// squash and stretch jump animations.
// real animations for everything!
// camera for moving around?

// I'm not sure I'll get to code today but here's a plan for my next prototype.
// Let's GENERATE a map (using the current scrolling camera)
// The goal is to get to the anti sun.
// There will be 1 powerup, a double jump.
// Generate a level where you have to get the double jump, then you can get to the antisun.
// This is going to be a hard problem actuially!

// Do I program the double jump first? Should I do double jump or super jump?

// zeeshan sent me a really dope paper on precedural level generation
// http://www.is.ovgu.de/is_media/Master+und+Bachelor_Arbeiten/MasterThesis_JensDieskau-p-2680.pdf
// Gotta check it out and see if it helps. It has some mcts shit going on which is super interesting!

extern crate nalgebra_glm as glm; // @TODO: Probably just write this ourselves.

use std::f64;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::console;

mod collide;
mod map;
mod platform;

use collide::{sweep_aabb, test_aabb, SweepResult};
use platform::Key;

const TICK: f64 = 1.0 / 60.0;

macro_rules! console_log {
    ($($t:tt)*) => (console::log_1(&JsValue::from_str(&format_args!($($t)*).to_string())))
}

#[derive(Debug)]
pub struct Aabb {
    center: glm::Vec2,
    extent: glm::Vec2,
}

#[derive(Debug)]
pub enum Geometry {
    AABB { aabb: Aabb },
}

#[derive(Debug)]
pub struct Input {
    pub up: bool,
    pub down: bool,
    pub left: bool,
    pub right: bool,
    pub jump: bool,

    pub view_map: bool,
}

#[derive(Debug)]
pub enum Color {
    DebugPink,
    Black,
    DarkPurple,
    DarkBlue,
    DarkGray,
    Gray,
    MediumBlue,
    LightBlue,
    White,
    LightSand,
    MediumSand,
    DarkSand,
    Rock,
    DarkRock,
    Red,
    Green,
    Blue,
}

#[derive(Debug)]
pub struct RenderRect {
    world_center: glm::Vec2,
    world_extent: glm::Vec2,
    color: Color,
}

#[derive(Debug)]
pub struct Renderer {
    rects: Vec<RenderRect>,
    collision_tiles: Vec<(usize, usize)>,
    debug_ray: glm::Vec2,
}

impl Renderer {
    // Position is bottom left.
    fn rect(&mut self, center: glm::Vec2, extent: glm::Vec2, color: Color) {
        self.rects.push(RenderRect {
            world_center: center,
            world_extent: extent,
            color,
        });
    }
}

#[derive(Debug, PartialEq)]
pub enum GameMode {
    Playing,
    ViewingTheMap,
}

pub struct Game {
    mode: GameMode,

    world: map::World,
    t: f64, // Game Time

    player_room_x: i32,
    player_room_y: i32,

    player_pressed_jump_at: f64,
    player_jumped_at: f64,
    player_p: glm::Vec2,
    player_dp: glm::Vec2,
    player_grounded: bool,
    player_last_grounded: f64, // Dunno if this would be quite right.

    player_can_pass: u8,

    collision_tiles: Vec<(usize, usize)>,
    debug_ray: glm::Vec2,
}

// @TODO: Friction and jump aren't data driven yet.

const SECONDS_PER_FRAME: f32 = 1.0 / 60.0;
const JUMP_MAX_HEIGHT: f32 = 4.0; // 3 tiles
const TIME_TO_MAX_HEIGHT: f32 = 36.0 * SECONDS_PER_FRAME; // 36 frames

const MAX_SPEED: f32 = 10.0; // 5 tiles per second.
const TIME_TO_MAX_SPEED: f32 = 6.0 * SECONDS_PER_FRAME; // 6 frames
const TIME_TO_STOP_FROM_MAX_SPEED: f32 = 3.0 * SECONDS_PER_FRAME; // 3 frames

const SPEED: f32 = MAX_SPEED / TIME_TO_MAX_SPEED;
const FRICTION: f32 = 10.0;

impl Game {
    pub fn new() -> Self {
        Self {
            mode: GameMode::Playing, //GameMode::ViewingTheMap,
            world: map::World::new(),
            t: 0.0,
            player_room_x: 0,
            player_room_y: 0,
            player_pressed_jump_at: 0.0,
            player_jumped_at: 0.0,
            player_grounded: false,
            player_last_grounded: 0.0,
            player_p: glm::vec2(5.1, 8.1),
            player_dp: glm::vec2(0.0, 0.0),
            player_can_pass: 0,
            collision_tiles: vec![],
            debug_ray: glm::vec2(0.0, 0.0),
        }
    }

    pub fn update(&mut self, input: &mut Input) {
        if let Some(tile_map) = self
            .world
            .rooms
            .get(&(self.player_room_x, self.player_room_y))
        {
            if input.view_map {
                match self.mode {
                    GameMode::Playing => self.mode = GameMode::ViewingTheMap,
                    GameMode::ViewingTheMap => self.mode = GameMode::Playing,
                };
                input.view_map = false;
            }

            if self.mode == GameMode::ViewingTheMap {
                return;
            }

            let dt = 1.0 / 60.0;
            self.collision_tiles.clear();

            let mut accel = glm::vec2(0.0, 0.0);
            if input.left {
                accel.x -= 1.0;
            }
            if input.right {
                accel.x += 1.0;
            }
            if self.player_can_pass >= 4 {
                if input.up {
                    accel.y += 1.0;
                }
                if input.down {
                    accel.y -= 1.0;
                }
            }
            if accel.magnitude() > 0.0 {
                accel = accel.normalize();
            }
            accel *= SPEED;

            // @NOTE: "reactivity"
            // this is a dot product or something.
            if (accel.x > 0.0 && self.player_dp.x < 0.0)
                || (accel.x < 0.0 && self.player_dp.x > 0.0)
            {
                accel.x += accel.x * 0.5; // reactivity percent
            }

            if input.jump {
                self.player_pressed_jump_at = self.t;
                input.jump = false; // @TODO: better way to do this?
            }
            if (self.t - self.player_pressed_jump_at < 0.15)
                && (self.player_grounded || self.t - self.player_last_grounded < 0.15)
            {
                accel.y += 2500.0;
                self.player_pressed_jump_at = 0.0;
                self.player_jumped_at = self.t + (dt as f64);
                self.player_last_grounded = 0.0;
            }
            // @TODO: Gravity
            if self.player_can_pass < 4 {
                accel.y -= 100.0;
            }

            let player_geometry = Aabb {
                center: self.player_p + glm::vec2(0.0, 1.0 - 0.01),
                extent: glm::vec2(0.5 - 0.02, 1.0 - 0.02),
            };
            let mut new_dp = accel * dt + self.player_dp;
            new_dp = new_dp / (1.0 + FRICTION * dt);

            self.player_dp = new_dp;
            if self.player_dp.x > MAX_SPEED {
                self.player_dp.x = MAX_SPEED;
            }
            if self.player_dp.x < -MAX_SPEED {
                self.player_dp.x = -MAX_SPEED;
            }

            let mut new_p = 0.5 * accel * (dt * dt) + self.player_dp * dt + self.player_p;
            let mut ray = new_p - self.player_p; // maybe / dt_remaining
            if ray.magnitude() > 0.0 {
                let magnitude = ray.magnitude() / dt;
                ray = ray.normalize() * magnitude;
            }

            let mut dt_remaining = dt;
            'time: while dt_remaining > 0.0 {
                let mut min_hit_t = std::f32::INFINITY;
                let mut hit_plane = glm::vec2(0.0, 0.0);

                'tiles: for (i, tile) in tile_map.iter().enumerate() {
                    let tile_y = (i / 32) as f32;
                    let tile_x = (i % 32) as f32;
                    if *tile > self.player_can_pass {
                        let tile_geometry = Aabb {
                            center: glm::vec2(tile_x as f32 + 0.5, 18.0 - (tile_y as f32 + 0.5)),
                            extent: glm::vec2(0.5, 0.5),
                        };
                        let SweepResult {
                            hit,
                            hit_time,
                            hit_normal,
                        } = sweep_aabb(&player_geometry, &tile_geometry, &ray, dt_remaining);

                        if hit {
                            self.collision_tiles
                                .push((tile_x as usize, tile_y as usize));
                            if hit_time < min_hit_t {
                                min_hit_t = hit_time;
                                hit_plane = hit_normal;
                            }
                        }
                    }
                }

                // Maybe we can track the player's actual movement.
                let moved_from;
                let moved_to;
                let mut should_break = false;

                // now we've been over every tile, was there a hit?
                if min_hit_t < std::f32::INFINITY {
                    dt_remaining -= min_hit_t;
                    moved_from = self.player_p;
                    self.player_p = self.player_p + ray * (min_hit_t - 0.0001);
                    moved_to = self.player_p;

                    if hit_plane.x != 0.0 {
                        self.player_dp.x = 0.0;
                        accel.x = 0.0;
                    } else if hit_plane.y != 0.0 {
                        self.player_dp.y = 0.0;
                        accel.y = 0.0;
                    }

                    new_p = 0.5 * accel * (dt_remaining * dt_remaining)
                        + self.player_dp * dt_remaining
                        + self.player_p;
                    ray = new_p - self.player_p;
                    if ray.magnitude() > 0.0 {
                        let magnitude = ray.magnitude() / dt_remaining;
                        ray = ray.normalize() * magnitude;
                    }
                } else {
                    moved_from = self.player_p;
                    self.player_p = self.player_p + ray * dt_remaining;
                    moved_to = self.player_p;
                    should_break = true;
                }

                // Here we can maybe see if we hit anything else.
                let ray = moved_to - moved_from;

                // @TODO: How can I drop the orb?
                if let Some(room_entities) = self
                    .world
                    .room_entities
                    .get(&(self.player_room_x, self.player_room_y))
                {
                    for orb in room_entities {
                        let orb_geometry = Aabb {
                            center: orb.pos,
                            extent: glm::vec2(0.5, 0.5),
                        };
                        let SweepResult { hit, .. } =
                            sweep_aabb(&player_geometry, &orb_geometry, &ray, 1.0);

                        if hit && orb.level > self.player_can_pass {
                            self.player_can_pass = orb.level;
                        }
                    }
                }

                if should_break {
                    break;
                }
            }

            self.player_dp = accel * dt + self.player_dp;

            // @NOTE: Screen scrolling.
            // @TODO: Don't do this if there's no room over there!
            if self.player_p.x > 32.0 {
                if let Some(room) = self
                    .world
                    .rooms
                    .get(&(self.player_room_x + 1, self.player_room_y))
                {
                    self.player_room_x += 1;
                    self.player_p.x -= 32.0
                } else {
                    self.player_p.x = 32.0
                }
            }
            if self.player_p.x < 0.0 {
                if let Some(room) = self
                    .world
                    .rooms
                    .get(&(self.player_room_x - 1, self.player_room_y))
                {
                    self.player_room_x -= 1;
                    self.player_p.x += 32.0
                } else {
                    self.player_p.x = 0.0
                }
            }
            if self.player_p.y > 18.0 {
                if let Some(room) = self
                    .world
                    .rooms
                    .get(&(self.player_room_x, self.player_room_y + 1))
                {
                    self.player_room_y += 1;
                    self.player_p.y -= 18.0
                } else {
                    self.player_p.y = 18.0
                }
            }
            if self.player_p.y < 0.0 {
                if let Some(room) = self
                    .world
                    .rooms
                    .get(&(self.player_room_x, self.player_room_y - 1))
                {
                    self.player_room_y -= 1;
                    self.player_p.y += 18.0
                } else {
                    self.player_p.y = 0.0
                }
            }

            self.t += TICK;

            // @Q: Should checking for groundedness happen before moving or after?
            let feet_geometry = Aabb {
                center: self.player_p,
                extent: glm::vec2(0.5, 0.1),
            };
            let ray = glm::vec2(0.0, 0.0); // @Q: Does this work?

            let mut grounded = false;

            // @TODO: When I loop tiles I need to only loop tiles by the player.
            // This probably means I want to store them in a different way.
            for (i, tile) in tile_map.iter().enumerate() {
                let tile_y = (i / 32) as f32;
                let tile_x = (i % 32) as f32;
                if *tile > self.player_can_pass {
                    let tile_geometry = Aabb {
                        center: glm::vec2(tile_x as f32 + 0.5, 18.0 - (tile_y as f32 + 0.5)),
                        extent: glm::vec2(0.4, 0.5),
                    };
                    let hit = test_aabb(&feet_geometry, &tile_geometry);

                    if hit {
                        grounded = true;
                        break;
                    }
                }
            }
            if grounded {
                self.player_grounded = true;
                self.player_last_grounded = self.t;
            } else {
                self.player_grounded = false;
            }
        }
    }

    pub fn render(&mut self, dt_remaining: f32, renderer: &mut Renderer) {
        // @TODO: Return draw lists or something of what to render.
        renderer.rects.clear();
        renderer.collision_tiles.clear();
        renderer.collision_tiles.extend(&self.collision_tiles);

        renderer.debug_ray = self.debug_ray;

        // I think I probably can make this work without altering my renderer though it's
        // a little bit hacky.
        if self.mode == GameMode::ViewingTheMap {
            // Draw out the map.
            for ((x, y), _room) in &self.world.rooms {
                let outer_color = Color::DebugPink;
                // let outer_color = match level {
                //     0 => Color::Rock,
                //     1 => Color::Red,
                //     2 => Color::Green,
                //     3 => Color::Blue,
                //     _ => Color::Black,
                // };

                renderer.rect(
                    glm::vec2((*x as f32) + 16.0, (*y as f32) + 9.0),
                    glm::vec2(0.5, 0.5),
                    outer_color,
                );

                renderer.rect(
                    glm::vec2((*x as f32) + 16.0, (*y as f32) + 9.0),
                    glm::vec2(0.4, 0.4),
                    Color::Gray,
                );

                if *x == self.player_room_x && *y == self.player_room_y {
                    // add the player to the room.
                    let color = Color::LightBlue;
                    renderer.rect(
                        glm::vec2((*x as f32) + 16.0, (*y as f32) + 9.0),
                        glm::vec2(0.25, 0.25),
                        color,
                    );
                }
            }

            for (((x1, y1), (x2, y2)), door) in &self.world.doors {
                let door_color = match door {
                    0 => Color::Rock,
                    1 => Color::Red,
                    2 => Color::Green,
                    3 => Color::Blue,
                    _ => Color::Black,
                };
                if x1 == x2 {
                    renderer.rect(
                        glm::vec2((*x1 as f32) + 16.0, (*y1 as f32) + 9.5),
                        glm::vec2(0.2, 0.2),
                        door_color,
                    );
                } else {
                    renderer.rect(
                        glm::vec2((*x1 as f32) + 16.5, (*y1 as f32) + 9.0),
                        glm::vec2(0.2, 0.2),
                        door_color,
                    );
                }
            }

            return;
        }

        if let Some(tile_map) = self
            .world
            .rooms
            .get(&(self.player_room_x, self.player_room_y))
        {
            // Tilemap
            for (i, tile) in tile_map.iter().enumerate() {
                let y = i / 32;
                let x = i % 32;
                if *tile > 0 {
                    let mut color = match tile {
                        1 => Color::Red,
                        2 => Color::Green,
                        3 => Color::Blue,
                        4 => Color::Rock,
                        5 => Color::Black,
                        _ => Color::DebugPink,
                    };

                    // for (colx, coly) in &renderer.collision_tiles {
                    //     if *colx == x && *coly == y {
                    //         color = Color::DebugPink;
                    //     }
                    // }

                    renderer.rect(
                        glm::vec2(x as f32 + 0.5, 18.0 - (y as f32 + 0.5)),
                        glm::vec2(0.5, 0.5),
                        color,
                    );
                }
            }

            if let Some(room_entities) = self
                .world
                .room_entities
                .get(&(self.player_room_x, self.player_room_y))
            {
                for orb in room_entities {
                    let color = match orb.level {
                        1 => Color::Red,
                        2 => Color::Green,
                        3 => Color::Blue,
                        _ => Color::Black,
                    };
                    renderer.rect(orb.pos, glm::vec2(0.5, 0.5), color);
                }
            }

            // Player
            let player_p = self.player_p + self.player_dp * dt_remaining;

            renderer.rect(
                player_p + glm::vec2(0.0, 1.0),
                glm::vec2(0.5, 1.0),
                Color::LightBlue,
            );
            if self.player_can_pass > 0 {
                renderer.rect(
                    player_p + glm::vec2(0.0, 1.0) + glm::vec2(0.0, 1.0),
                    glm::vec2(0.4, 0.25),
                    Color::Red,
                );
            }
            if self.player_can_pass > 1 {
                renderer.rect(
                    player_p + glm::vec2(0.0, 1.0) + glm::vec2(0.0, 1.25),
                    glm::vec2(0.3, 0.25),
                    Color::Green,
                );
            }
            if self.player_can_pass > 2 {
                renderer.rect(
                    player_p + glm::vec2(0.0, 1.0) + glm::vec2(0.0, 1.5),
                    glm::vec2(0.2, 0.25),
                    Color::Blue,
                );
            }

            // debug collisions with ground
            let ground_color = match self.player_grounded {
                true => Color::DarkBlue,
                false => Color::DebugPink,
            };
            renderer.rect(
                player_p + glm::vec2(0.0, 0.0),
                glm::vec2(0.4, 0.1),
                ground_color,
            );
        }
    }
}

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();
    Ok(())
}

#[wasm_bindgen]
pub struct Platform {
    dpr: f64,
    canvas: web_sys::HtmlCanvasElement,
    ctx: web_sys::CanvasRenderingContext2d,
    last_t: f64,
    dt: f64,
    // game stuff
    renderer: Renderer,
    input: Input,
    game: Game,
}

#[wasm_bindgen]
impl Platform {
    pub fn new() -> Result<Platform, JsValue> {
        console_log!("Setting up Game");

        let document = web_sys::window().unwrap().document().unwrap();
        let canvas = document.get_element_by_id("canvas").unwrap();
        let canvas: web_sys::HtmlCanvasElement = canvas
            .dyn_into::<web_sys::HtmlCanvasElement>()
            .map_err(|_| ())
            .unwrap();

        let ctx = canvas
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into::<web_sys::CanvasRenderingContext2d>()
            .unwrap();

        let dpr = web_sys::window().unwrap().device_pixel_ratio();
        console::log_1(&JsValue::from_str(&format!("Dpr: {}", dpr)));

        let last_t = 0.0;
        let dt = 0.0;

        let game = Game::new();

        let input = Input {
            //@TODO: Input handling is more subdle than this.
            up: false,
            down: false,
            left: false,
            right: false,
            jump: false,
            view_map: false,
        };

        let renderer = Renderer {
            rects: vec![],
            collision_tiles: vec![],
            debug_ray: glm::vec2(0.0, 0.0),
        };

        Ok(Self {
            dpr,
            canvas,
            ctx,
            last_t,
            dt,
            input,
            renderer,
            game,
        })
    }

    pub fn onkey(&mut self, key: u32, pressed: bool) {
        let key: Key = key.into();
        match key {
            Key::W => self.input.up = pressed,
            Key::S => self.input.down = pressed,
            Key::A => self.input.left = pressed,
            Key::D => self.input.right = pressed,
            Key::M => self.input.view_map = pressed,
            Key::Space => {
                if pressed {
                    self.input.jump = true
                }
            }
            _ => (),
        };
    }

    pub fn update(&mut self, t: f64) -> Result<(), JsValue> {
        // duration in seconds
        let mut dt = self.dt + f64::min(1.0, (t - self.last_t) / 1000.0);
        while dt > TICK {
            dt = dt - TICK;
            self.game.update(&mut self.input);
        }
        self.dt = dt;
        self.input.jump = false;

        self.game.render(dt as f32, &mut self.renderer);

        let display_width = self.canvas.client_width() as u32 * self.dpr as u32;
        let display_height = self.canvas.client_height() as u32 * self.dpr as u32;

        if self.canvas.width() != display_width || self.canvas.height() != display_height {
            self.canvas.set_width(display_width);
            self.canvas.set_height(display_height);
            self.ctx.scale(self.dpr, self.dpr)?;
            console_log!("Resizing");
        }

        let width = self.canvas.client_width() as f64;
        let height = self.canvas.client_height() as f64;
        let aspect_ratio = width / height;

        // tile size
        let ts = width / 32.0;
        // half tile size
        //let hts = ts / 2.0;

        // Assumes we're 16 x 9
        // @TODO: This is a bad way to check this.
        assert_eq!((aspect_ratio * 100.0) as i64, 177);

        self.ctx.clear_rect(0.0, 0.0, width, height);

        self.ctx.save();
        // @Q: Why can this fail?
        // Now we have 0,0 in the bottom left.
        // @TODO: This will all depend on the camera or whatever.
        self.ctx.translate(0.0, height)?;

        for rect in &self.renderer.rects {
            self.ctx.save();
            // @TODO: Cache these don't make strings every frame.
            // https://lospec.com/palette-list/copper-tech
            match &rect.color {
                Color::DebugPink => self.ctx.set_fill_style(&JsValue::from_str("pink")),
                Color::Black => self.ctx.set_fill_style(&JsValue::from_str("#000000")),
                Color::DarkPurple => self.ctx.set_fill_style(&JsValue::from_str("#262144")),
                Color::DarkBlue => self.ctx.set_fill_style(&JsValue::from_str("#355278")),
                Color::DarkGray => self.ctx.set_fill_style(&JsValue::from_str("#60748a")),
                Color::Gray => self.ctx.set_fill_style(&JsValue::from_str("#898989")),
                Color::MediumBlue => self.ctx.set_fill_style(&JsValue::from_str("#5aa8b2")),
                Color::LightBlue => self.ctx.set_fill_style(&JsValue::from_str("#91d9f3")),
                Color::White => self.ctx.set_fill_style(&JsValue::from_str("#ffffff")),
                Color::LightSand => self.ctx.set_fill_style(&JsValue::from_str("#f4cd72")),
                Color::MediumSand => self.ctx.set_fill_style(&JsValue::from_str("#bfb588")),
                Color::DarkSand => self.ctx.set_fill_style(&JsValue::from_str("#c58843")),
                Color::Rock => self.ctx.set_fill_style(&JsValue::from_str("#9e5b47")),
                Color::DarkRock => self.ctx.set_fill_style(&JsValue::from_str("#5f4351")),
                Color::Red => self.ctx.set_fill_style(&JsValue::from_str("#dc392d")),
                Color::Green => self.ctx.set_fill_style(&JsValue::from_str("#6ea92c")),
                Color::Blue => self.ctx.set_fill_style(&JsValue::from_str("#1651dd")),
            };

            let x = (rect.world_center.x - rect.world_extent.x) as f64 * ts;
            let y = (rect.world_center.y + rect.world_extent.y) as f64 * ts;
            let width = (rect.world_extent.x * 2.0) as f64 * ts;
            let height = (rect.world_extent.y * 2.0) as f64 * ts;

            self.ctx.fill_rect(x, -y, width, height);

            self.ctx.restore();
        }

        self.ctx.restore();

        let x = ts;
        let mut y = 1.5 * ts;
        self.ctx
            .set_font(&format!("{}px Georgia", (ts / 2.0) as i32));
        self.ctx.fill_text("Debug text can go here.", x, y)?;
        y += ts / 2.0;
        self.ctx.fill_text(
            &format!(
                "fps: {}, ms per frame: {}",
                f64::round(1000.0 / (t - self.last_t)),
                t - self.last_t
            ),
            x,
            y,
        )?;

        self.last_t = t;

        Ok(())
    }
}
