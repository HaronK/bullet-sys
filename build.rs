extern crate bindgen;
extern crate cc;
extern crate walkdir;

mod build_linux;

use std::env;
use std::path::PathBuf;

fn main() {
    //if cfg!(feature = "build") {
    if env::var("TARGET").unwrap().contains("linux") {
        build_linux::build(false);
    } else {
        panic!("Unsupported platform!");
    }
    //}

    //if cfg!(feature = "bind") {
    println!("cargo:rustc-link-lib=bullet3");

    let bindings = bindgen::Builder::default()
        .clang_arg("-x")
        .clang_arg("c++")
        .clang_arg("-Ibullet3/src")
        .header("bullet3.h")
        .generate()
        .expect("Unable to generate bindings");

    //let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    let out_path = PathBuf::from("src/");
    bindings
        .write_to_file(out_path.join("bullet3.rs"))
        .expect("Couldn't write bindings!");
    //}
}
