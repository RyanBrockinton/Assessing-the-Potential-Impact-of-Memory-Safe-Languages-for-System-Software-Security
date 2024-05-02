// Execute with ./heapRust a_20bytes_1234567890

use std::env;

const BUFSIZE: usize = 16;

fn print_heap_data(buf1: &[u8], buf2: &[u8]) {
    let buf1_str = std::str::from_utf8(buf1).unwrap_or_default().trim_end_matches('\0');
    let buf2_str = std::str::from_utf8(buf2).unwrap_or_default().trim_end_matches('\0');
    println!("Heap Data 1: {:?}, Memory Address: {:?}", buf1_str, buf1.as_ptr());
    println!("Heap Data 2: {:?}, Memory Address: {:?}", buf2_str, buf2.as_ptr());
}

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("argv[1]: {}", args[1]);

    let mut buf1 = vec![0u8; BUFSIZE];
    let buf2 = vec![0u8; BUFSIZE];
    buf1[..8].copy_from_slice(b"password");

    println!("\nExecute:\nlet mut buf1 = vec![0u8; {}];\nlet buf2 = vec![0u8; {}];\nbuf1[..8].copy_from_slice(b\"password\");\n", BUFSIZE, BUFSIZE);

    print_heap_data(&buf1, &buf2);

    println!("\nExecute:\nbuf1[..args[1].len()].copy_from_slice(args[1].as_bytes());\n");
    buf1[..args[1].len()].copy_from_slice(args[1].as_bytes());
    
    print_heap_data(&buf1, &buf2);
}