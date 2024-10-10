// Copyright (c) 2023 Nick Piaddo
// SPDX-License-Identifier: Apache-2.0 OR MIT

use std::env;
use std::path::PathBuf;

mod parser;

fn main() {
    // Use the system's `pkg-config` utility to find the location of the libblkid library for
    // linking with the `ld` command
    let library = match pkg_config::Config::new()
        .print_system_libs(false)
        .probe("blkid")
    {
        Ok(lib) => lib,
        Err(e) => panic!("run pkg_config failed: {:?}", e),
    };

    //  Determine libblkid's version.
    let mut gcc = cc::Build::new();

    for include_dir in library.include_paths.iter() {
        gcc.include(include_dir);
    }

    let str_expansion = match gcc.flag("-O").file("build/version_probe.c").try_expand() {
        Ok(expanded) => expanded,
        Err(e) => {
            panic!(
                "
Header expansion error:
{:?}

Failed to find development headers for libblkid. Please the util-linux headers package specific to
your Linux distribution:

    # On Alpine Linux
    apk add util-linux-dev
    # On Arch Linux
    sudo pacman -S util-linux-libs
    # On Nixos
    nix-env -iA nixos.util-linux.dev
    # On Ubuntu
    sudo apt-get install libblkid-dev
                ",
                e
            );
        }
    };

    // Generate version information in DEP_XXX_VERSION_NUMBER environment variable (where XXX is the
    // name of the package having `rsblkid-sys` as a dependency).
    let prefix = "RUST_BLKID_VERSION_";
    let str_expansion = String::from_utf8(str_expansion).unwrap();

    for line in str_expansion.lines() {
        let line = line.replace(['"', '\\'], "");

        if let Some(stripped) = line.trim().strip_prefix(prefix) {
            let version = parser::parse_version(stripped);

            println!("cargo:version_number={:x}", version);
        }
    }

    // Add libblkid's headers to Rust's include paths.
    for include in library.include_paths.iter() {
        println!("cargo:include={}", include.display());
    }

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
