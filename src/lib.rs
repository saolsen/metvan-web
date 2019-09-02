use std::f64;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::console;

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
}

#[derive(Debug)]
pub struct Game {
    p: V2,
    dp: V2,
}

impl Game {
    pub fn new() -> Self {
        Self {
            p: V2::zero(),
            dp: V2::zero(),
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

#[derive(Debug)]
pub enum Key {
    Unknown,
    Backspace,
    Tab,
    Return,
    Esc,
    Space,
    PageUp,
    PageDown,
    End,
    Home,
    Left,
    Right,
    Down,
    Insert,
    Delete,
    Zero,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    I,
    J,
    K,
    L,
    M,
    N,
    O,
    P,
    Q,
    R,
    S,
    T,
    U,
    V,
    W,
    X,
    Y,
    Z,
    Tilda,
}

impl From<u32> for Key {
    fn from(v: u32) -> Key {
        match v {
            8 => Key::Backspace,
            9 => Key::Tab,
            13 => Key::Return,
            27 => Key::Esc,
            32 => Key::Space,
            33 => Key::PageUp,
            34 => Key::PageDown,
            35 => Key::End,
            36 => Key::Home,
            37 => Key::Left,
            39 => Key::Right,
            40 => Key::Down,
            45 => Key::Insert,
            46 => Key::Delete,
            48 => Key::Zero,
            49 => Key::One,
            50 => Key::Two,
            51 => Key::Three,
            52 => Key::Four,
            53 => Key::Five,
            54 => Key::Six,
            55 => Key::Seven,
            56 => Key::Eight,
            57 => Key::Nine,
            65 => Key::A,
            66 => Key::B,
            67 => Key::C,
            68 => Key::D,
            69 => Key::E,
            70 => Key::F,
            71 => Key::G,
            72 => Key::H,
            73 => Key::I,
            74 => Key::J,
            75 => Key::K,
            76 => Key::L,
            77 => Key::M,
            78 => Key::N,
            79 => Key::O,
            80 => Key::P,
            81 => Key::Q,
            82 => Key::R,
            83 => Key::S,
            84 => Key::T,
            85 => Key::U,
            86 => Key::V,
            87 => Key::W,
            88 => Key::X,
            89 => Key::Y,
            90 => Key::Z,
            192 => Key::Tilda,
            _ => Key::Unknown,
        }
    }
}

#[wasm_bindgen]
pub struct Platform {
    dpr: f64,
    canvas: web_sys::HtmlCanvasElement,
    ctx: web_sys::CanvasRenderingContext2d,
    // game stuff
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

        let game = Game::new();
        console_log!("{:?}", game);

        let input = Input {
            left: false,
            right: false,
        };

        Ok(Self {
            dpr,
            canvas,
            ctx,
            input,
            game,
        })
    }

    pub fn onkey(&mut self, key: u32, pressed: bool) {
        let key: Key = key.into();
        match key {
            Key::A => self.input.left = pressed,
            Key::D => self.input.right = pressed,
            _ => (),
        };
        console_log!("{:?}: pressed?: {}", key, pressed);
    }

    pub fn update(&mut self, _t: f64) -> Result<(), JsValue> {
        //self.game.p.x = 0.0;
        //self.game.p.y = 0.0;

        if self.input.left {
            self.game.p.x -= 1.0;
        }

        if self.input.right {
            self.game.p.x += 1.0;
        }

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
