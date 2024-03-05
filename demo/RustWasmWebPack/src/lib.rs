use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;


// This is like the `main` function, except for JavaScript.
#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {

    console_error_panic_hook::set_once();


    let window = web_sys::window().unwrap();
    let document = window
        .document()
        .unwrap();

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

    sierpinski_triangle(&context, [(300.0, 0.0), (150.00, 300.0), (450.0, 300.0)], 9);

    Ok(())
}

fn draw_triangle(context: &web_sys::CanvasRenderingContext2d, points: [(f64, f64); 3]){
    let [top, left, rigth] = points;
    context.move_to(top.0, top.1);
    context.begin_path();
    context.line_to(left.0, left.1);
    context.line_to(rigth.0, rigth.1);
    context.line_to(top.0, top.1);
    context.close_path();
    context.stroke(); 
}

fn sierpinski_triangle(context: &web_sys::CanvasRenderingContext2d, points: [(f64, f64); 3], depth: u8){
    draw_triangle(&context, points);

    let depth = depth - 1;

    let [top, left, rigth] = points;

    if depth > 0 {
        let left_middle = midpoint(top, left);
        let rigth_middle = midpoint(top, rigth);
        let botton_middle = midpoint(left, rigth);

        sierpinski_triangle(&context, [top, left_middle, rigth_middle], depth);
        sierpinski_triangle(&context, [left_middle, left, botton_middle], depth);
        sierpinski_triangle(&context, [rigth_middle, botton_middle, rigth], depth);
    }
}

fn midpoint(point_1: (f64, f64), point_2: (f64, f64)) -> (f64, f64){
    ((point_1.0 + point_2.0) / 2.0, (point_1.1 + point_2.1) / 2.0)
}