use std::mem;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct RawString {
    pub pointer: *const u8,
    pub length: usize,
}

#[wasm_bindgen(js_name = makeString)]
pub fn make_string() -> RawString {
    // Create an owned string
    let string = String::from("Raw string from Rust!");

    // Get raw pointer and length
    let pointer = string.as_ptr();
    let length = string.len();

    // Prevent string from being freed
    mem::forget(string);

    // Return the raw parts of the string wrapped in a struct
    RawString { pointer, length }
}
