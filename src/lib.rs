use wasm_bindgen::prelude::*;
use web_sys::window;
use wasm_bindgen::JsCast;

#[wasm_bindgen]
pub fn convert(name: &str) {
    let greetings = format!("Hello, {}!", name);

    let window = window().unwrap();
    let document = window.document().unwrap();
    let target = document.get_element_by_id("target").expect("document should have a target element");

    let img = document.create_element("img")
    .unwrap()
    .dyn_into::<web_sys::HtmlImageElement>()
    .unwrap();
    img.set_alt(&greetings);
    img.set_name("output");

    target.append_child(&img).unwrap();
}
