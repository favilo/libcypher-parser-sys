use std::{env, path::PathBuf};

fn main() {
    println!("cargo:rerun-if-changed=wrapper.h");

    pkg_config::Config::new()
        .atleast_version("0.6.0")
        // .statik(true)
        .probe("cypher-parser")
        .expect("Need to install libcypher-parser-dev package");
    println!("cargo:rustc-link-lib=static=cypher-parser");

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
        .opaque_type("std.*")
        .whitelist_type("cypher.*")
        .whitelist_function("cypher.*")
        .whitelist_var("cypher.*")
        .whitelist_var("CYPHER.*")
        // Set enumerations to generate rust version
        .rustified_enum("cypher_rel_direction")
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
