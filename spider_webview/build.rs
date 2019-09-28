use std::env;
use std::path::PathBuf;

fn main() {
    // Generate the bridge library Rust bindings
    let bindings_file = PathBuf::from(env::var("OUT_DIR").unwrap()).join("bridge.rs");
    bindgen::Builder::default()
        .header("bridge/lib.h")
        .generate()
        .expect("Unable to generate bindings")
        .write_to_file(&bindings_file)
        .expect("Couldn't write bindings");
    println!("{:?}", bindings_file);
    // Compile the bridge library
    cc::Build::new()
        .files(&["bridge/macos/lib.m"])
        .include("bridge")
        .compile("bridge");
    // Link the appropriate frameworks
    for framework in &["Cocoa", "WebKit"] {
        println!("cargo:rustc-link-lib=framework={}", framework);
    }
}
