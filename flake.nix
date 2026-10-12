{
  description = "Rust Flake Template";

  inputs = {
    nixpkgs.url      = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url  = "github:numtide/flake-utils";
  };

  outputs = { nixpkgs, rust-overlay, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };
        libPath = if pkgs.stdenv.isLinux then with pkgs; lib.makeLibraryPath [
          libGL
          libxkbcommon
          wayland
          wayland
          libX11
          libXcursor
          libXi
          libXrandr
        ] else "";
      in
      {
        devShells.default = with pkgs; mkShell {
          buildInputs = [
            pkg-config
            openssl
            nodejs
            pkgsCross.mingwW64.stdenv.cc
            pkgsCross.mingwW64.stdenv.cc.bintools
            # wasm-pack
            cargo-flamegraph
            rust-analyzer
            rustfmt
            # wasm-bindgen-cli
            # Keep this version in sync with rust-toolchain.toml, which pins the
            # same toolchain for rustup users (CI + non-nix contributors).
            (rust-bin.nightly."2026-04-01".default.override {
              # targets = [ "wasm32-unknown-unknown" ];
              extensions = ["rust-analyzer"];
            })
          ] ++ lib.optionals stdenv.isLinux [ libxcb perf ];
          LD_LIBRARY_PATH = libPath;
        };
      }
    );
}
