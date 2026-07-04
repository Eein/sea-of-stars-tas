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
        libPath = with pkgs; lib.makeLibraryPath [
          libGL
          libxkbcommon
          wayland
          wayland
          libX11
          libXcursor
          libXi
          libXrandr
        ];
      in
      {
        devShells.default = with pkgs; mkShell {
          buildInputs = [
            pkg-config
            openssl
            nodejs
            libxcb
            perf
            # wasm-pack
            cargo-flamegraph
            rust-analyzer
            rustfmt
            # wasm-bindgen-cli
            (rust-bin.nightly.latest.default.override {
              # targets = [ "wasm32-unknown-unknown" ];
              extensions = ["rust-analyzer"];
            })
          ];
          LD_LIBRARY_PATH = libPath;
        };
      }
    );
}
