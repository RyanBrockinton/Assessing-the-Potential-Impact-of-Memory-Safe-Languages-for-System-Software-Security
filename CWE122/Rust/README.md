## Native Rust file
1. Compile: ```rustc [xxx.rs]```
2. Execute: ```./[xxx] [args]```

## WebAssembly Rust file
1. Create template ```cargo new --lib [directory_name]```.
2. ```cd [directory_name]```
3. Copy ```Cargo.toml```, ```src/lib.rs``` then replace ones in template.
4. Compile: ```wasm-pack build --target web```
5. Copy ```index.html``` then paste in ```[directory_name]```
6. Launch the html file.