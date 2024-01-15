use rand::{rngs::ThreadRng, thread_rng, Rng};
use wasm_bindgen::prelude::*;
use web_sys::console;

fn draw_triangle(
    context: &web_sys::CanvasRenderingContext2d,
    start_x: f64,
    start_y: f64,
    width: f64,
    height: f64,
    color: (u8, u8, u8),
) {
    context.begin_path();
    context.move_to(start_x, start_y);
    context.line_to(start_x - width, start_y + height);
    context.line_to(start_x + width, start_y + height);
    context.close_path();
    context.stroke();
    let color_str = format!("rgb({},{},{})", color.0, color.1, color.2);
    context.set_fill_style(&wasm_bindgen::JsValue::from_str(&color_str));
    context.fill();
}

fn sierpinski(
    context: &web_sys::CanvasRenderingContext2d,
    start_x: f64,
    start_y: f64,
    width: f64,
    height: f64,
    depth: u32,
) -> () {
    let mut rng = thread_rng();
    let next_color = (
        rng.gen_range(0..255),
        rng.gen_range(0..255),
        rng.gen_range((0..255)),
    );
    draw_triangle(context, start_x, start_y, width, height, next_color);
    if depth > 0 {
        sierpinski(
            context,
            start_x,
            start_y,
            width / 2.0,
            height / 2.0,
            depth - 1,
        );
        sierpinski(
            context,
            start_x + width / 2.0,
            start_y + height / 2.0,
            width / 2.0,
            height / 2.0,
            depth - 1,
        );
        sierpinski(
            context,
            start_x - width / 2.0,
            start_y + height / 2.0,
            width / 2.0,
            height / 2.0,
            depth - 1,
        );
    }
}

fn draw_stem(context: &web_sys::CanvasRenderingContext2d) {
    context.begin_path();
    context.move_to(250.0, 450.0);
    context.line_to(350.0, 450.0);
    context.line_to(350.0, 600.0);
    context.line_to(250.0, 600.0);
    context.close_path();
    context.stroke();
    let color_str = "rgb(116,80,48)";
    context.set_fill_style(&wasm_bindgen::JsValue::from_str(&color_str));
    context.fill();
}

// This is like the `main` function, except for JavaScript.
#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    // This provides better error messages in debug mode.
    // It's disabled in release mode so it doesn't bloat up the file size.
    console_error_panic_hook::set_once();

    // Your code goes here!
    console::log_1(&JsValue::from_str("Hello world!"));

    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let canvas = document
        .get_element_by_id("canvas")
        .unwrap()
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .unwrap();
    let context = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .unwrap();
    let interval_callback = Closure::wrap(Box::new(move || {
        sierpinski(&context, 300.0, 0.0, 256.0, 450.0, 8);
        draw_stem(&context);
    }) as Box<dyn FnMut()>);
    window.set_interval_with_callback_and_timeout_and_arguments_0(
        interval_callback.as_ref().unchecked_ref(),
        500,
    );
    interval_callback.forget();
    Ok(())
}
