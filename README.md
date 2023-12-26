# rsblkid-sys

![Crates.io License](https://img.shields.io/crates/l/rsblkid-sys?labelColor=%23222222&color=%230d0887)
![Crates.io MSRV](https://img.shields.io/crates/msrv/rsblkid-sys?labelColor=%23222222&color=%239c179e)

----

Raw Rust FFI bindings to the [`util-linux/libblkid`][1] C library.

----

## Supported library version

This crate requires `libblkid` version `2.39.2` or later.

## Build dependencies

Install the following packages beforehand to build this crate:

- `util-linux`: to generate Rust bindings from `libblkid`'s header files.
- `libclang`: to satisfy the [dependency][2] of [`bindgen`][3] on `libclang`.
- `pkg-config`: to detect system libraries.

This in addition to the [Rust toolchain][4].

### NixOS

This repository contains a configuration file (`flake.nix`) that will
automatically set up a development environment for you. If you have [configured
your system to use flakes][5], issue the following command from the directory
containing the `flake.nix` file.

```console
nix develop
```

The flake will take care of installing the Rust toolchain. Once package
downloads and installations are complete, compile the crate with the command
`cargo build`.

Alternately, you can manually install the required packages temporarily with:

```console
nix-shell -p util-linux.dev libclang.lib pkg-config
```

or permanently with:

```console
nix-env -iA nixos.util-linux.dev nixos.libclang.lib nixos.pkg-config
```

### Alpine Linux

As `root`, issue the following command:

```console
apk add util-linux-dev clang-libclang pkgconfig
```

Install the Rust toolchain via [`rustup`][4] or the package manager, then run
`cargo build`:

```console
apk add cargo
```

## License

Copyright (c) 2023 Nick Piaddo

SPDX-License-Identifier: Apache-2.0 OR MIT

[1]: https://github.com/util-linux/util-linux/tree/master
[2]: https://rust-lang.github.io/rust-bindgen/requirements.html#clang
[3]: https://crates.io/crates/bindgen
[4]: https://www.rust-lang.org/tools/install
[5]: https://nixos.wiki/wiki/flakes
