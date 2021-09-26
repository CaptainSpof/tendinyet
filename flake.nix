{
  description = "Arms are cool, lets keep them that way.";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, rust-overlay, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pname = "tendinyet";
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs { inherit system overlays; };
        rust = pkgs.rust-bin.nightly.latest.default.override {
          extensions = [ "rust-src" ];
        };
      in {
        devShell = pkgs.mkShell {

          buildInputs = with pkgs; [
            gcc
            rust
            rust-analyzer
          ];

          shellHook = ''
            echo "Welcome to ${pname} !"
            # [ ! -f ./target/$CARGO_BUILD_TARGET/debug/${pname} ] && cargo build ; ln -sf ./target/$CARGO_BUILD_TARGET/debug/${pname} ${pname}
          '';
        };
      });
}
