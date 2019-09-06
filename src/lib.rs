extern crate nalgebra_glm as glm;

use std::f64;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::console;

mod platform;

use platform::Key;

const TICK: f64 = 1.0 / 60.0;

macro_rules! console_log {
    ($($t:tt)*) => (console::log_1(&JsValue::from_str(&format_args!($($t)*).to_string())))
}

#[derive(Debug)]
pub struct Input {
    pub left: bool,
    pub right: bool,
    pub jump: bool,
}

#[derive(Debug)]
pub struct Renderer {}

#[derive(Debug)]
pub struct Game {
    t: f64, // Game Time

    player_p: glm::Vec2,
    player_dp: glm::Vec2,
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
            player_p: glm::vec2(0.0, 0.0),
            player_dp: glm::vec2(0.0, 0.0),
        }
    }

    pub fn update(&mut self, input: &Input) {
        let dt = 1.0 / 60.0;
        // What we want are rigid body dynamics.
        let mut accel = glm::vec2(0.0, 0.0);
        if input.left {
            accel.x -= 1.0;
        }
        if input.right {
            accel.x += 1.0;
        }

        // @TODO: This is in pixels right now. Don't
        let speed = 50.0;
        accel *= speed;
        // @TODO: Better friction
        accel.x += -5.0 * self.player_dp.x;
        // @NOTE: "reactivity"
        // @TODO: I really need better vectors...
        // this is a dot product or something.
        if (accel.x > 0.0 && self.player_dp.x < 0.0) || (accel.x < 0.0 && self.player_dp.x > 0.0) {
            accel.x += accel.x * 0.5; // reactivity percent
        }

        if input.jump {
            accel.y += 1000.0;
        }

        // @TODO: Gravity
        accel.y -= 50.0;

        let mut new_p = 0.5 * accel * (dt * dt) + self.player_dp * dt + self.player_p;
        let mut new_dp = accel * dt + self.player_dp;

        if new_p.y < 0.0 {
            new_p.y = 0.0;
            new_dp.y = 0.0;
        }
        self.player_dp = new_dp;
        self.player_p = new_p;
        // @TODO: Collision Detection
        // @TODO: Everything
        self.t += TICK;
    }

    pub fn render(&mut self, _dt_left: f64, _renderer: &mut Renderer) {
        // @TODO: Return draw lists or something of what to render.
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
            left: false,
            right: false,
            jump: false,
        };

        let renderer = Renderer {};

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

        // @TODO: Gravity!

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
        let ts = width / 16.0;
        // half tile size
        let hts = ts / 2.0;

        // assumes we're 16 x 9
        // @TODO: This is a bad way to check this.
        assert_eq!((aspect_ratio * 100.0) as i64, 177);

        self.ctx.clear_rect(0.0, 0.0, width, height);

        // self.ctx.fill_rect(0.0, 0.0, hts, hts);
        // self.ctx.fill_rect(width - hts, 0.0, hts, hts);
        // self.ctx.fill_rect(width - hts, height - hts, hts, hts);
        // self.ctx.fill_rect(0.0, height - hts, hts, hts);

        self.ctx.save();
        // @Q: Why can this fail?
        // Now we have 0,0 in the center of the screen
        self.ctx.translate(width / 2.0, height * 5.0 / 6.0)?;

        // Draw Floor
        self.ctx.fill_rect(-width / 2.0, 0.0, width, 10.0);

        // Draw character
        self.ctx.save();
        self.ctx.translate(
            self.game.player_p.x as f64 * ts,
            -self.game.player_p.y as f64 * ts,
        )?;

        self.ctx.set_fill_style(&JsValue::from_str("black"));
        self.ctx.begin_path();
        // self.ctx
        //     .arc(0.0, 0.0, hts / 2.0, 0.0, f64::consts::PI * 2.0)?;
        self.ctx.set_fill_style(&JsValue::from_str("red"));
        self.ctx.fill_rect(-hts / 2.0, -ts, hts, ts);
        self.ctx.stroke();
        self.ctx.restore();

        self.ctx.restore();

        Ok(())
    }
}

// If whole screen is 16x9, dude is like 2 tall.
// That's a good starting point. prolly not exactly right.
