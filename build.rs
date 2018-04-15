extern crate cmake;
extern crate bindgen;

use std::env;
use std::path::PathBuf;
use cmake::Config;

fn main() {
    let bindings = bindgen::Builder::default()
        .clang_arg("-Ivendor/enet/include/")
        .header("wrapper.h")
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");

    
    let dst = Config::new("vendor/enet")
                .cflag("-DENET_DEBUG")
                .build_target("enet")
                .build();

    eprintln!("LUL: {}", dst.display());

    println!("cargo:rustc-link-search=native={}/build", dst.display());
    println!("cargo:rustc-link-lib=static=enet");
}
