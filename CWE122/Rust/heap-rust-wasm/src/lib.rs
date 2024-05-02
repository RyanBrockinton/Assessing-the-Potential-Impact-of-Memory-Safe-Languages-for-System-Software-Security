use wasm_bindgen::prelude::*;

const BUFSIZE: usize = 16;


fn heap_data(buf: &Vec<u8>) -> String {
    let buf_str = std::str::from_utf8(buf).unwrap_or_default().trim_end_matches('\0');
    let data = format!("Stack Data: {:?}, Memory Address: {:?}", buf_str, buf.as_ptr());
    data
}

#[wasm_bindgen]
pub fn heap_test(arg: &str) -> Vec<String>{
    let mut result = Vec::new();

    let mut buf1 = vec![0u8; BUFSIZE];
    let buf2 = vec![0u8; BUFSIZE];
    buf1[..8].copy_from_slice(b"password");

    result.push(format!("Execute:\nlet mut buf1 = vec![0u8; {}];\nlet buf2 = vec![0u8; {}];\nbuf1[..8].copy_from_slice(b\"password\");\n", BUFSIZE, BUFSIZE));

    result.push(heap_data(&buf1));
    result.push(heap_data(&buf2));

    result.push(format!("Execute:\nbuf1[..arg.len()].copy_from_slice(arg.as_bytes());\n"));

    buf1[..arg.len()].copy_from_slice(arg.as_bytes());

    result.push(heap_data(&buf1));
    result.push(heap_data(&buf2));

    result
}