extern crate cbindgen;

use std::env;

fn main() {
    cbindgen::generate(env::var("CARGO_MANIFEST_DIR").unwrap())
        .expect("Unable to generate bindings")
        .write_to_file("packetlib.h");
}
