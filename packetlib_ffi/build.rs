extern crate cbindgen;

use std::env;

use cbindgen::{Config, Language};

fn main() {
    let _ = std::fs::create_dir("include");
    let crate_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let mut config = Config::from_root_or_default(&crate_dir);
    config.cython.header = Some(String::from("\"packetlib.h\""));
    let builder_c = cbindgen::Builder::new()
        .with_config(config)
        .with_crate(crate_dir);
    let builder_cpy = builder_c
        .clone()
        .with_language(Language::Cython)
        .with_autogen_warning("");
    builder_c
        .generate()
        .expect("Unable to generate C bindings")
        .write_to_file("include/packetlib.h");
    builder_cpy
        .generate()
        .expect("Unable to generate Cython bindings")
        .write_to_file("include/packetlib.pxd");
    csbindgen::Builder::default()
        .input_extern_file("src/protocol.rs")
        .input_extern_file("src/lib.rs")
        .input_extern_file("src/connection.rs")
        .input_extern_file("src/ppac.rs")
        .csharp_namespace("packetlib")
        .csharp_dll_name("packetlib_ffi")
        .generate_csharp_file("include/packetlib.cs")
        .expect("Unable to generate C# bindings");
}
