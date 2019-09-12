// Next Steps
// Use the raycast collision detection for actual movement updates.
// Figure out where we intersect so I can manage the velocity.
// minkowski sums so we're having the whole player geometry hit.
// use extra dt on render
// better jump code

// I will probably go full vector graphics world with minkowski but this part is the same either way.

extern crate nalgebra_glm as glm; // @TODO: Probably just write this ourselves.

use std::f64;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::console;

mod platform;

use platform::Key;

// For testing
// bottom left is 0,0
const TILE_MAP: [u8; 32 * 18] = [
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
    1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
    1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
    1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
    1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
    1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
    1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
    1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 4, 4, 4, 0, 0, 0, 0, 1,
    1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 4, 4, 4, 4, 4, 0, 0, 1,
    1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 4, 4, 4, 4, 4, 0, 0, 1,
    1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
    1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
    1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
    1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
    1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 2, 0, 0, 0, 3, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
    1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 2, 0, 0, 0, 3, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
    1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 2, 0, 0, 0, 3, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
];

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
}

#[derive(Debug)]
pub enum Color {
    Brown,
    LightGreen,
    LightBlue,
    Black,
    Green,
    Red,
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

#[derive(Debug)]
pub struct Game {
    t: f64, // Game Time

    player_jumped_at: f64,
    player_p: glm::Vec2,
    player_dp: glm::Vec2,

    collision_tiles: Vec<(usize, usize)>,
    debug_ray: glm::Vec2,
}

// How can we figure out the physics numbers.

// I think I want the max jump height to be 3.
// That would let me have a really big jump.

// I think all my parameters have to come from calculations
// about what I want my max jump height to be and my max speed or something like that.

impl Game {
    pub fn new() -> Self {
        Self {
            t: 0.0,
            player_jumped_at: 0.0,
            player_p: glm::vec2(5.1, 5.1),
            player_dp: glm::vec2(0.0, 0.0),
            collision_tiles: vec![],
            debug_ray: glm::vec2(0.0, 0.0),
        }
    }

    pub fn update(&mut self, input: &Input) {
        let dt = 1.0 / 60.0;
        self.collision_tiles.clear();

        // What we want are rigid body dynamics.
        let mut accel = glm::vec2(0.0, 0.0);
        if input.left {
            accel.x -= 1.0;
        }
        if input.right {
            accel.x += 1.0;
        }
        if input.up {
            accel.y += 1.0;
        }
        if input.down {
            accel.y -= 1.0;
        }
        if accel.magnitude() > 0.0 {
            accel = accel.normalize();
        }
        let speed = 1000.0;
        accel *= speed;
        // @TODO: Better friction
        //accel.x += -5.0 * self.player_dp.x;
        // @NOTE: "reactivity"
        // @TODO: I really need better vectors...
        // this is a dot product or something.
        if (accel.x > 0.0 && self.player_dp.x < 0.0) || (accel.x < 0.0 && self.player_dp.x > 0.0) {
            //accel.x += accel.x * 0.5; // reactivity percent
        }
        // @NOTE: Not the way to do this. Probably check landings and stuff.
        // if input.jump && self.t - self.player_jumped_at > 1.0 {
        if input.jump {
            accel.y += 10.0;
            self.player_jumped_at = self.t + (dt as f64);
        }
        // @TODO: Gravity
        //accel.y -= 0.1;

        let mut new_dp = accel * dt + self.player_dp;

        let player_geometry = Aabb {
            center: self.player_p + glm::vec2(0.0, 1.0 - 0.01),
            extent: glm::vec2(0.5, 1.0 - 0.02),
        };

        // @NOTE: Just checking for collisions.
        for (i, tile) in TILE_MAP.iter_mut().enumerate() {
            let y = (i / 32) as f32;
            let x = (i % 32) as f32;
            if *tile > 0 {
                let tile_geometry = Aabb {
                    center: glm::vec2(x as f32 + 0.5, 18.0 - (y as f32 + 0.5)),
                    extent: glm::vec2(0.5, 0.5),
                };

                let a = &player_geometry;
                let b = &tile_geometry;
                let collides = if ((a.center.x - b.center.x).abs() > (a.extent.x + b.extent.x))
                    || ((a.center.y - b.center.y).abs() > (a.extent.y + b.extent.y))
                {
                    false
                } else {
                    true
                };

                if collides {
                    //self.collision_tiles.push((x as usize, y as usize));
                    //console_log!("collision: ({},{})", x, y);
                }
            }
        }

        // How does this algorithm work? It's a raycast of my position against the tiles,
        // but it's actually a minkowski sum thing becuase I'm an aabb too. Then I want to
        // know the hit normal and the point on the ray that hits because I will
        // move that far and cancel out the motion perpendicular to the normal or whatever
        // but then keep going. We'll use the center of the aabb, not the position because
        // that's how the math will work out with minkowski sums.

        // @Q: Do I do this math in dt or in 0-1?
        //if new_p - self.player_p != glm::vec2(0.0, 0.0) {
        let mut dt_remaining = dt;
        let mut ray_o = self.player_p; // origin
        let mut new_p = 0.5 * accel * (dt * dt) + self.player_dp * dt + self.player_p;
        let mut ray_d = new_p - ray_o; // maybe / dt_remaining
        if ray_d.magnitude() > 0.0 {
            let magnitude = ray_d.magnitude() / dt_remaining;
            ray_d = ray_d.normalize() * magnitude;
        }
        let mut hit_plane = glm::vec2(0.0, 0.0);

        //assert_eq!(ray_o + ray_d * dt_remaining, new_p);

        self.debug_ray = glm::vec2(ray_d.x, ray_d.y);

        'time: while dt_remaining > 0.0 {
            'tiles: for (i, tile) in TILE_MAP.iter_mut().enumerate() {
                let y = (i / 32) as f32;
                let x = (i % 32) as f32;
                if *tile > 0 {
                    let tile_geometry = Aabb {
                        center: glm::vec2(x as f32 + 0.5, 18.0 - (y as f32 + 0.5)),
                        extent: glm::vec2(0.5, 0.5),
                    };

                    // @OPTIMIZATION: You can do a version of this without the branches.
                    // see https://tavianator.com/fast-branchless-raybounding-box-intersections-part-2-nans/

                    let bmin_x =
                        tile_geometry.center.x - tile_geometry.extent.x - player_geometry.extent.x;
                    let bmax_x =
                        tile_geometry.center.x + tile_geometry.extent.x + player_geometry.extent.x;
                    let bmin_y = tile_geometry.center.y
                        - tile_geometry.extent.y
                        - player_geometry.extent.y * 2.0;
                    let bmax_y = tile_geometry.center.y + tile_geometry.extent.y;

                    let mut tmin = std::f32::NEG_INFINITY;
                    let mut tmax = std::f32::INFINITY;

                    if ray_d.x != 0.0 {
                        let tx1 = (bmin_x - ray_o.x) / ray_d.x;
                        let tx2 = (bmax_x - ray_o.x) / ray_d.x;

                        let min_x = f32::min(tx1, tx2);
                        if min_x > tmin {
                            tmin = min_x;
                            hit_plane = glm::vec2(1.0, 0.0);
                        }
                        let max_x = f32::max(tx1, tx2);
                        if max_x < tmax {
                            tmax = max_x;
                        }
                    //tmin = f32::max(tmin, f32::min(tx1, tx2));
                    //tmax = f32::min(tmax, f32::max(tx1, tx2));
                    } else if ray_o.x <= bmin_x || ray_o.x >= bmax_x {
                        continue 'tiles; // return false.
                    }
                    if ray_d.y != 0.0 {
                        let ty1 = (bmin_y - ray_o.y) / ray_d.y;
                        let ty2 = (bmax_y - ray_o.y) / ray_d.y;

                        let min_y = f32::min(ty1, ty2);
                        if min_y > tmin {
                            tmin = min_y;
                            hit_plane = glm::vec2(0.0, 1.0);
                        }
                        let max_y = f32::max(ty1, ty2);
                        if max_y < tmax {
                            tmax = max_y;
                        }
                    // tmin = f32::max(tmin, f32::min(ty1, ty2));
                    // tmax = f32::min(tmax, f32::max(ty1, ty2));
                    } else if ray_o.y <= bmin_y || ray_o.y >= bmax_y {
                        continue 'tiles; // return false.
                    }

                    if tmax >= tmin {
                        // @TODO
                        // We need the hit time and the normal of the hit, then we can
                        // update our position, cancel our some movement and keep going.
                        if tmin >= 0.0 && tmin < dt_remaining {
                            // We hit this thing at time tmin.
                            self.collision_tiles.push((x as usize, y as usize));

                            dt_remaining -= tmin;
                            ray_o += ray_d * (tmin - 0.1);
                            if hit_plane.x == 1.0 {
                                ray_d.x = 0.0;
                                new_dp.x = 0.0;
                            } else if hit_plane.y == 1.0 {
                                ray_d.y = 0.0;
                                new_dp.y = 0.0;
                            }
                            break 'tiles;
                        }
                    }
                }
            }
            // No collisions happened.
            new_p = ray_o + ray_d * dt_remaining;
            break;
        }
        if hit_plane.x == 1.0 {
            new_dp.x = 0.0;
        } else if hit_plane.y == 1.0 {
            new_dp.y = 0.0;
        }
        self.player_dp = new_dp;
        self.player_p = new_p;
        // @TODO: Everything
        self.t += TICK;
    }

    pub fn render(&mut self, _dt_left: f64, renderer: &mut Renderer) {
        console_log!("{:?}", self.player_p);
        // @TODO: Return draw lists or something of what to render.
        renderer.rects.clear();
        renderer.collision_tiles.clear();
        renderer.collision_tiles.extend(&self.collision_tiles);

        renderer.debug_ray = self.debug_ray;

        // Tilemap
        for (i, tile) in TILE_MAP.iter_mut().enumerate() {
            let y = i / 32;
            let x = i % 32;
            if *tile > 0 {
                let mut color = match tile {
                    1 => Color::Brown,
                    2 => Color::LightGreen,
                    3 => Color::LightBlue,
                    _ => Color::Black,
                };

                for (colx, coly) in &renderer.collision_tiles {
                    if *colx == x && *coly == y {
                        color = Color::Red;
                    }
                }

                renderer.rect(
                    glm::vec2(x as f32 + 0.5, 18.0 - (y as f32 + 0.5)),
                    glm::vec2(0.5, 0.5),
                    color,
                );
            }
        }

        // Player
        // @TODO: Use remaining dt for this.
        //console_log!("player: ({},{})", self.player_p.x, self.player_p.y);
        renderer.rect(
            self.player_p + glm::vec2(0.0, 1.0),
            glm::vec2(0.5, 1.0),
            Color::Green,
        );
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
        console_log!("{:?}", game);

        let input = Input {
            //@TODO: Input handling is more subdle than this.
            up: false,
            down: false,
            left: false,
            right: false,
            jump: false,
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
            self.game.update(&self.input);
        }
        self.dt = dt;
        self.last_t = t;
        self.input.jump = false;

        // @TODO: There is still dt time left over
        // Use that to interpolate stuff when rendering.
        self.game.render(dt, &mut self.renderer);

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
            match &rect.color {
                Color::Brown => self.ctx.set_fill_style(&JsValue::from_str("brown")),
                Color::LightGreen => self.ctx.set_fill_style(&JsValue::from_str("lightgreen")),
                Color::LightBlue => self.ctx.set_fill_style(&JsValue::from_str("lightblue")),
                Color::Black => self.ctx.set_fill_style(&JsValue::from_str("black")),
                Color::Green => self.ctx.set_fill_style(&JsValue::from_str("green")),
                Color::Red => self.ctx.set_fill_style(&JsValue::from_str("red")),
            };

            let x = (rect.world_center.x - rect.world_extent.x) as f64 * ts;
            let y = (rect.world_center.y + rect.world_extent.y) as f64 * ts;
            let width = (rect.world_extent.x * 2.0) as f64 * ts;
            let height = (rect.world_extent.y * 2.0) as f64 * ts;

            self.ctx.fill_rect(x, -y, width, height);

            self.ctx.restore();
        }

        self.ctx.restore();

        Ok(())
    }
}
