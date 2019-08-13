#[no_mangle]
pub extern "C" fn add_rust(x: i32, y: i32) -> i32 {
    x + y
}
