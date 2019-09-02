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

#[derive(Debug, Copy, Clone)]
pub struct V2 {
    pub x: f32,
    pub y: f32,
}

impl V2 {
    pub fn zero() -> Self {
        V2 { x: 0.0, y: 0.0 }
    }
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

    p: V2,
    dp: V2,
}

impl Game {
    pub fn new() -> Self {
        Self {
            t: 0.0,
            p: V2::zero(),
            dp: V2::zero(),
        }
    }

    pub fn update(&mut self, input: &Input) {
        if input.left {
            self.p.x -= 1.0;
        }

        if input.right {
            self.p.x += 1.0;
        }

        if input.jump {
            self.p.y += 20.0;
        }

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

        self.ctx.clear_rect(0.0, 0.0, width, height);

        self.ctx.fill_rect(10.0, 10.0, 10.0, 10.0);
        self.ctx.fill_rect(width - 20.0, 10.0, 10.0, 10.0);
        self.ctx.fill_rect(width - 20.0, height - 20.0, 10.0, 10.0);
        self.ctx.fill_rect(10.0, height - 20.0, 10.0, 10.0);

        self.ctx.save();
        self.ctx
            .translate(self.game.p.x as f64, -self.game.p.y as f64)?;

        // @Q: Why can this fail?
        self.ctx.translate(width / 2.0, height / 2.0)?;
        // Now we have 0,0 in the center of the screen
        self.ctx.set_fill_style(&JsValue::from_str("black"));
        self.ctx.begin_path();
        self.ctx.arc(0.0, 0.0, 5.0, 0.0, f64::consts::PI * 2.0)?;
        self.ctx.set_fill_style(&JsValue::from_str("red"));
        self.ctx.fill_rect(-5.0, -20.0, 10.0, 20.0);
        self.ctx.stroke();

        self.ctx.restore();

        self.input.jump = false;

        Ok(())
    }
}
