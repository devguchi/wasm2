use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn run() -> Result<(), JsValue> {
    let window = web_sys::window().expect("no blobal 'window' exists");
    let document = window.document().expect("should have a document on window");
    let body = document.body().expect("document should have a body");
    let p = document.create_element("p")?;
    p.set_text_content(Some("Hello from Rust!"));
    body.append_child(&p)?;
    Ok(())
}