use std::env;
use std::path::PathBuf;

fn main() {
    if let Ok(path) = env::var("LIBRARY_PATH") {
        println!("cargo:rustc-link-search=native={}", path);
    }

    if cfg!(target_os = "macos") {
        // We're linking against libcue statically, a static archive
        // is just a collection of object files, unlike a shared object
        // it doesn't specify its dynamic dependencies. Add the system
        // shared libraries that libcue depends on to the linker search
        // path. We only have to do this on macOS because on other
        // Unix systems (and Windows) Go binaries only depend on libc
        // (or equivalent), which cargo links with already.
        println!("cargo:rustc-link-lib=framework=CoreFoundation");
        println!("cargo:rustc-link-lib=framework=Security");
    }

    println!("cargo:rustc-link-lib=static=cue");

    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        // Invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
