extern crate cbindgen;

use std::env;

fn main() {
    cbindgen::generate(env::var("CARGO_MANIFEST_DIR").unwrap())
        .expect("Unable to generate bindings")
        .write_to_file("packetlib.h");
    csbindgen::Builder::default()
        .input_extern_file("src/lib.rs")
        .input_extern_file("src/protocol.rs")
        .input_extern_file("src/connection.rs")
        .csharp_namespace("packetlib")
        .csharp_dll_name("packetlib_ffi")
        .generate_csharp_file("packetlib.cs")
        .unwrap();
}
