{
  description = "Basic devshell for polybase";
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay.url = "github:oxalica/rust-overlay";
    noir.url = "github:noir-lang/noir";
    barretenberg.url = "github:AztecProtocol/barretenberg";
  };

  outputs = { nixpkgs, flake-utils, rust-overlay, barretenberg, ... } @ inputs:
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

      in {
        devShells.default = pkgs.mkShell {
          # inputsFrom = [ noir.devShells.${system}.default ];
          packages = with pkgs; [
            rustToolchain

            pkg-config
            openssl
            
            noir.packages."${system}".default
          ];

          RUST_SRC_PATH = "${rustToolchain}/lib/rustlib/src/rust/library";
        };
      });
}
