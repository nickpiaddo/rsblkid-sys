{
  description = "Raw Rust FFI bindings to util-linux/libblkid";

  inputs = {
    # Nixpkgs / NixOS version to use.
    nixpkgs.url = "nixpkgs/nixos-23.11";

    # Set of functions to make flake nix packages simpler to set up without
    # external dependencies.
    utils.url = "github:numtide/flake-utils";

    # Nix library for building Rust projects
    naersk.url = "github:nix-community/naersk/master";

    # Backward compatibility for people without flakes enabled.
    # https://github.com/edolstra/flake-compat
    flake-compat.url = "https://flakehub.com/f/edolstra/flake-compat/1.tar.gz";
  };

  outputs = { self, nixpkgs, utils, naersk, flake-compat }:
    utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs { inherit system; };
        naersk-lib = pkgs.callPackage naersk { };
      in rec
      {
        packages.default = naersk-lib.buildPackage ./.;

        # Development environment
        devShells.default = with pkgs; mkShell {
          buildInputs = [
            # For Markdown
            glow
            pandoc
            lynx
            w3m

            # Rust
            cargo
            cargo-audit
            cargo-flamegraph
            cargo-modules
            cargo-nextest
            cargo-vet
            cargo-valgrind
            cargo-workspaces
            lldb
            rustc
            rust-analyzer
            rustfmt
            rustPackages.clippy

            # For code linting and formatting
            nodejs_20
            marksman
            pre-commit
            ruby
            shellcheck
            shfmt
          ];

          # Rust source path
          RUST_SRC_PATH = rustPlatform.rustLibSrc;
        };
      });
}
