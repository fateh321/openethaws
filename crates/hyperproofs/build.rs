// fn main() {
//     println!("cargo:rustc-link-search=native={}", "./hyperproofs-go");
//     println!("cargo:rustc-link-search=native={}", "/usr/local/lib");
//     println!("cargo:rustc-link-lib=static={}", "shard");
// }
extern crate bindgen;

use std::env;
use std::path::PathBuf;

fn main() {
    // Tell cargo to look for shared libraries in the specified directory
    // println!("cargo:rustc-link-search=native={}", "./hyperproofs-go");
    println!("cargo:rustc-link-search=native={}", "/data/ubuntu/libhyper/hyperproofs-go");
    // println!("cargo:rustc-env=LD_LIBRARY_PATH=hyperproofs-go/");
    // println!("cargo:rustc-link-search=native={}", "/usr/local/lib");
    // Tell cargo to tell rustc to link the system bzip2
    // shared library.
    // println!("cargo:rustc-link-lib=static={}","shard");
    println!("cargo:rustc-link-lib={}","shard");
    // println!("cargo:rustc-link-lib=static={}","mcl");

    // Tell cargo to invalidate the built crate whenever the wrapper changes
    println!("cargo:rerun-if-changed=wrapper.h");

    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let bindings = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .header("wrapper.h")
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}