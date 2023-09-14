{
  description = "Basic devshell for polybase";
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay.url = "github:oxalica/rust-overlay";
    noir.url = "github:noir-lang/noir";
  };

  outputs = { nixpkgs, flake-utils, rust-overlay, ... } @ inputs:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [ (import rust-overlay) ];
        };
        noir = (import inputs.noir);

        rustToolchain = pkgs.rust-bin.stable."1.70.0".default.override {
          extensions = [ "rust-src" "rust-analyzer" ];
        };

        barretenberg = pkgs.fetchFromGitHub {
            owner = "AztecProtocol";
            repo = "barretenberg";
            rev = "fdd46f77531a6fcc9d9b24a698c56590d54d487e";
            sha256 = "sha256-w7yMeYp50KrlTn23TTKfYmLOQL4uIgw0wSX67v2tvvc=";
        };
        libbarretenberg = pkgs.callPackage "${barretenberg}/barretenberg.nix" {};

      in {
        devShells.default = pkgs.mkShell {
          packages = with pkgs; [
#            rustToolchain

            pkg-config
            openssl

            noir.packages."${system}".noir-native
          ];

          RUST_SRC_PATH = "${rustToolchain}/lib/rustlib/src/rust/library";
        };
      });
}
