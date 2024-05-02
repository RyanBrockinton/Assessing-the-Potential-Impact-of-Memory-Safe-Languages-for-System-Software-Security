// Demonstrates safe Rust function using &mut i32.
// This function takes advantage of Rust's ownership and borrowing rules to ensure memory safety.
fn safe_rust_function(value: &mut i32) {
    println!("\nsafe_rust_function:");
    let ref_pointer: &mut i32 = value;
    println!("Address of ref_pointer: {:p}", ref_pointer);
    // Change address of ref_pointer to 0X0
    // This line is not allowed in safe Rust
    // ref_pointer = &mut *std::ptr::null_mut()
    println!("Value of ref_pointer: {}", *ref_pointer);
}

// Demonstrates unsafe Rust function using *mut i32.
// This function uses raw pointers and is marked as unsafe because it bypasses Rust's safety checks.
// Safer Rust code: define a unsafe function instead of using unsafe block
// i.e.unsafe fn unsafe_rust_function(value: *mut i32)
fn unsafe_rust_function(value: *mut i32) {
    println!("\nunsafe_rust_function:");
    // Below is unsafe block
    unsafe {
        let mut raw_pointer: *mut i32 = value;
        println!("Address of raw_pointer: {:p}", raw_pointer);
        println!("Value of raw_pointer: {}", *raw_pointer);
        // Change address of raw_pointer to 0X0
        raw_pointer = std::ptr::null_mut();
        println!("Execute:\nraw_pointer = std::ptr::null_mut();");
        println!("Address of raw_pointer: {:p}", raw_pointer);
        println!("Value of raw_pointer: {}", *raw_pointer);
    }
}

fn main() {
    let mut value = 10;

    safe_rust_function(&mut value);

    unsafe_rust_function(&mut value as *mut i32);

    // Safer Rust code instad of using unsafe block:
    // unsafe{
    //     unsafe_rust_function(&mut value as *mut i32);
    // }
}
