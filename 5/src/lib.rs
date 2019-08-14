mod utils;

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen(start)]
pub fn start() {
    utils::set_panic_hook();
    // panic!("example panic");

    let callback = Closure::wrap(Box::new(add_message) as Box<dyn Fn()>);

    let document = web_sys::window().unwrap().document().unwrap();
    let button = document
        .create_element("button")
        .unwrap()
        .dyn_into::<web_sys::HtmlButtonElement>()
        .unwrap();
    button.set_inner_html("Click me");
    document.body().unwrap().append_child(&button).unwrap();
    button.set_onclick(Some(callback.as_ref().unchecked_ref()));

    callback.forget();
}

fn add_message() {
    let document = web_sys::window().unwrap().document().unwrap();
    let element = document.create_element("p").unwrap();
    element.set_inner_html("Hello from Rust closure!");
    document.body().unwrap().append_child(&element).unwrap();
}
