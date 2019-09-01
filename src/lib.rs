use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::console;

// When the `wee_alloc` feature is enabled, this uses `wee_alloc` as the global
// allocator.
//
// If you don't want to use `wee_alloc`, you can safely delete this.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// This is like the `main` function, except for JavaScript.
#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    // This provides better error messages in debug mode.
    // It's disabled in release mode so it doesn't bloat up the file size.
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();

    // Your code goes here!
    console::log_1(&JsValue::from_str("This is Metvan!"));

    Ok(())
}

#[wasm_bindgen]
pub struct Game {
    canvas: web_sys::HtmlCanvasElement,
    ctx: web_sys::CanvasRenderingContext2d,
}

#[wasm_bindgen]
impl Game {
    pub fn new() -> Result<Game, JsValue> {
        console::log_1(&JsValue::from_str("Setting up Game"));

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

        Ok(Self { canvas, ctx })
    }

    pub fn update(&mut self, _t: f64) -> Result<(), JsValue> {
        let display_width = self.canvas.client_width() as u32;
        let display_height = self.canvas.client_height() as u32;

        if self.canvas.width() != display_width || self.canvas.height() != display_height {
            self.canvas.set_width(display_width);
            self.canvas.set_height(display_height);
            console::log_1(&JsValue::from_str("Resizing from rust"));
        }

        let width = display_width as f64;
        let height = display_height as f64;

        self.ctx.fill_rect(10.0, 10.0, 10.0, 10.0);
        self.ctx.fill_rect(width - 20.0, 10.0, 10.0, 10.0);
        self.ctx.fill_rect(width - 20.0, height - 20.0, 10.0, 10.0);
        self.ctx.fill_rect(10.0, height - 20.0, 10.0, 10.0);

        Ok(())
    }
}
