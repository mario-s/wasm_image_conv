use wasm_bindgen::prelude::*;
use web_sys::window;


#[wasm_bindgen]
pub fn convert(name: &str) {
    let greetings = &format!("Hello, {}!", name);

    let window = window().unwrap();
    let document = window.document().unwrap();
    let target = document.get_element_by_id("target").unwrap();
    target.set_text_content(Some(&greetings));
}
