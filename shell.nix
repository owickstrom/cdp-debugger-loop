let
  rust-overlay = (import (builtins.fetchTarball
    "https://github.com/oxalica/rust-overlay/archive/master.tar.gz"));
in { pkgs ? import <nixpkgs> { overlays = [ rust-overlay ]; } }:
let
  rust = pkgs.rust-bin.stable.latest.default.override {
    extensions = [ "rust-src" ];
  };
in pkgs.mkShell {
  buildInputs = [
    pkgs.bashInteractive
    rust
    pkgs.rust-analyzer
  ];
  RUST_SRC_PATH = "${rust}/lib/rustlib/src/rust/library";
}
