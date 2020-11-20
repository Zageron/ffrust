extern crate cfg_if;
extern crate wasm_bindgen;

mod utils;

use cfg_if::cfg_if;
use wasm_bindgen::prelude::*;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};

cfg_if! {
    // When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
    // allocator.
    if #[cfg(feature = "wee_alloc")] {
        extern crate wee_alloc;
        #[global_allocator]
        static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
    }
}

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);

    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(&format!("Hello, {}!", name));
}

#[wasm_bindgen]
pub fn start_game(canvas: HtmlCanvasElement, context: CanvasRenderingContext2d) {
    let width = canvas.width();
    let height = canvas.height();

    let divisions = 6;
    let square_width = width / divisions;
    let square_height = height / divisions;

    for x in 0..6 {
        for y in 0..6 {
            let color = 
                &format!(
                    "rgb({},{},{})",
                    (255.0 - 20.5 * x as f64).floor(),
                    (255.0 - 42.5 * y as f64).floor(),
                    255,
                ).into();

            context.set_fill_style(color);
            log(&format!("{:?} @ {}, {}, {}, {}", color, x * square_width, y * square_height, square_width, square_height));
            context.fill_rect(
                f64::from(x * square_width),
                f64::from(y * square_height),
                f64::from(square_width),
                f64::from(square_height),
            );
        }
    }
}
