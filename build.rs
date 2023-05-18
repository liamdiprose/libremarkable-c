// extern crate cbindgen;

use std::env;

fn main() {
    let crate_dir = env::var("CARGO_MANIFEST_DIR").unwrap();

    // println!("cargo:rerun-if-changed=src/lib.rs");
    // println!("cargo:rerun-if-changed=cbindgen.toml");

    // cbindgen::Builder::new()
    //   .with_config(cbindgen::Config::from_file("cbindgen.toml").expect("Unable to open cbindgen config"))
    //   .with_crate(crate_dir)
    //   .generate()
    //   .expect("Unable to generate bindings")
    //   .write_to_file("bindings.h");
}
