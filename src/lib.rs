use std::f64;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

#[wasm_bindgen(start)]
pub fn start() {
    let document = web_sys::window().unwrap().document().unwrap();
    let canvas = document.get_element_by_id("canvas").unwrap();
    let canvas: web_sys::HtmlCanvasElement = canvas
      .dyn_into::<web_sys::HtmlCanvasElement>()
      .unwrap();

    let context = canvas
      .get_context("2d")
      .unwrap()
      .unwrap()
      .dyn_into::<web_sys::CanvasRenderingContext2d>()
      .unwrap();

    context.begin_path();

    context
      .arc(75.0, 75.0, 50.0, 0.0, f64::consts::PI*2.0)
      .unwrap();

    context.move_to(110.0, 75.0);
    context.arc(75.0, 75.0, 35.0, 0.0, f64::consts::PI).unwrap();

    context.stroke();
}