use wasm_bindgen::prelude::*;

#[wasm_bindgen]
// Demonstrates safe Rust function using &mut i32.
// This function takes advantage of Rust's ownership and borrowing rules to ensure memory safety.
pub fn safe_rust_function() -> Vec<String> {
    let mut value = 10;
    let mut output: Vec<String> = Vec::new();
    output.push(String::from("\nsafe_rust_function\n----------------------"));
    let ref_pointer: &mut i32 = &mut value;
    output.push(format!("Address of ref_pointer: {:p}", ref_pointer));
    // Change address of ref_pointer to 0X0
    // This line is not allowed in safe Rust
    // ref_pointer = &mut *std::ptr::null_mut()
    output.push(format!("Value of ref_pointer: {}", *ref_pointer));
    output
}

#[wasm_bindgen]
// Demonstrates unsafe Rust function using *mut i32.
// This function uses raw pointers and is marked as unsafe because it bypasses Rust's safety checks.
pub unsafe fn unsafe_rust_function() -> Vec<String> {
    let mut value = 10;
    let mut output: Vec<String> = Vec::new();
    output.push(String::from("\nunsafe_rust_function\n----------------------"));
    let mut raw_pointer: *mut i32 = &mut value as *mut i32;
    output.push(format!("Address of raw_pointer: {:p}", raw_pointer));
    output.push(format!("Value of ref_pointer: {}", *raw_pointer));
    // Change address of raw_pointer to 0X0
    raw_pointer = std::ptr::null_mut();
    output.push(String::from("Execute:\nraw_pointer = std::ptr::null_mut();"));
    output.push(format!("Address of raw_pointer: {:p}", raw_pointer));
    output.push(format!("Value of ref_pointer: {}", *raw_pointer));
    output
}