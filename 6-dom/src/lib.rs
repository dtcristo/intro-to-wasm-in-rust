mod utils;

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{window, HtmlButtonElement};

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen(start)]
pub fn start() {
    utils::set_panic_hook();
    // panic!("example panic");

    // Create callback function
    let callback = Closure::wrap(Box::new(add_message) as Box<dyn Fn()>);

    // Create a button and append to DOM
    let document = window().unwrap().document().unwrap();
    let button = document
        .create_element("button")
        .unwrap()
        .dyn_into::<HtmlButtonElement>()
        .unwrap();
    button.set_inner_html("Click me");
    document.body().unwrap().append_child(&button).unwrap();

    // Add a callback on button
    button.set_onclick(Some(callback.as_ref().unchecked_ref()));

    // Prevent callback from being freed
    callback.forget();
}

fn add_message() {
    // Create a paragraph and append to DOM
    let document = window().unwrap().document().unwrap();
    let element = document.create_element("p").unwrap();
    element.set_inner_html("Hello from Rust callback!");
    document.body().unwrap().append_child(&element).unwrap();
}
