// Copyright (c) 2023 Nick Piaddo
// SPDX-License-Identifier: Apache-2.0 OR MIT

use std::env;
use std::path::PathBuf;

fn main() {
    // Use the syutem's `pkg-config` utility to find the location of the libblkid library for
    // linking with the `ld` command
    pkg_config::Config::new().probe("blkid").unwrap();

    // Tell cargo to tell rustc to link the system's `blkid` shared library.
    println!("cargo:rustc-link-lib=blkid");

    // Tell cargo to invalidate the built crate whenever the wrapper changes
    println!("cargo:rerun-if-changed=src/wrapper.h");

    // Tell cargo to invalidate the built crate whenever the build.rs changes
    println!("cargo:rerun-if-changed=build.rs");

    let bindings = bindgen::Builder::default()
        .header("src/wrapper.h")
        .allowlist_function("blkid_.*")
        .allowlist_var("BLKID_.*")
        .blocklist_item("BLKID_PROBREQ_.*")
        .clang_arg("-fretain-comments-from-system-headers")
        // Set the default type signedness to be used for macro constants
        .default_macro_constant_type(bindgen::MacroTypeVariation::Signed)
        // String constants should be generated as &CStr instead of &[u8]
        .generate_cstr(true)
        // Always translate enum integer types to native Rust integer types
        .translate_enum_integer_types(true)
        .generate()
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
