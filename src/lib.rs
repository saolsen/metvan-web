use std::f64;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::console;

macro_rules! console_log {
    ($($t:tt)*) => (console::log_1(&JsValue::from_str(&format_args!($($t)*).to_string())))
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
pub struct Game {
    dpr: f64,
    canvas: web_sys::HtmlCanvasElement,
    ctx: web_sys::CanvasRenderingContext2d,
}

#[wasm_bindgen]
impl Game {
    pub fn new() -> Result<Game, JsValue> {
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

        Ok(Self { dpr, canvas, ctx })
    }

    pub fn update(&mut self, _t: f64) -> Result<(), JsValue> {
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

        self.ctx.fill_rect(10.0, 10.0, 10.0, 10.0);
        self.ctx.fill_rect(width - 20.0, 10.0, 10.0, 10.0);
        self.ctx.fill_rect(width - 20.0, height - 20.0, 10.0, 10.0);
        self.ctx.fill_rect(10.0, height - 20.0, 10.0, 10.0);

        self.ctx.save();

        // @Q: Why can this fail?
        self.ctx.translate(width / 2.0, height / 2.0)?;
        // Now we have 0,0 in the center of the screen
        self.ctx.set_fill_style(&JsValue::from_str("red"));
        self.ctx.fill_rect(-5.0, -20.0, 10.0, 20.0);
        self.ctx.set_fill_style(&JsValue::from_str("black"));
        self.ctx.begin_path();
        self.ctx.arc(0.0, 0.0, 5.0, 0.0, f64::consts::PI * 2.0)?;
        self.ctx.stroke();

        self.ctx.restore();

        Ok(())
    }
}
