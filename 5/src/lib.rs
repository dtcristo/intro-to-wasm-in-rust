mod utils;

use wasm_bindgen::prelude::*;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    utils::set_panic_hook();
    // panic!("example panic");

    // https://rustwasm.github.io/wasm-bindgen/api/web_sys/
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let body = document.body().expect("document should have a body");

    let element = document.create_element("p")?;
    element.set_inner_html("Hello from Rust!");

    body.append_child(&element)?;

    Ok(())
}
