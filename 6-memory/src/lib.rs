mod utils;

use std::mem;
use wasm_bindgen::prelude::*;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen(start)]
pub fn start() {
    utils::set_panic_hook();
}

#[wasm_bindgen]
pub struct RawString {
    pub pointer: *const u8,
    pub length: usize,
}

#[wasm_bindgen(js_name = makeString)]
pub fn make_string() -> RawString {
    let string = String::from("Raw string from Rust!");

    // Get raw pointer and len
    let pointer = string.as_ptr();
    let length = string.len();

    // Prevent string from being freed
    mem::forget(string);

    RawString { pointer, length }
}
