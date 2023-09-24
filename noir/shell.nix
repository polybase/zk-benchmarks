{
    pkgs ? import <nixpkgs> {},
    mkShell ? pkgs.mkShell,
    callPackage ? pkgs.callPackage,
    fetchFromGitHub ? pkgs.fetchFromGitHub,
}: let
    barretenberg = fetchFromGitHub {
        owner = "AztecProtocol";
        repo = "barretenberg";
        rev = "fdd46f77531a6fcc9d9b24a698c56590d54d487e";
        sha256 = "sha256-w7yMeYp50KrlTn23TTKfYmLOQL4uIgw0wSX67v2tvvc=";
    };
    libbarretenberg = callPackage "${barretenberg}/barretenberg.nix" {};
in mkShell {
    buildInputs = [
        libbarretenberg
        pkgs.libiconv
        pkgs.llvmPackages.openmp
        pkgs.llvmPackages.libllvm
        pkgs.llvmPackages.libclang
        pkgs.llvmPackages.clang
        pkgs.pkg-config
    ] ++ pkgs.lib.optionals pkgs.stdenv.isDarwin [ pkgs.darwin.apple_sdk.frameworks.Security ];

    LIBCLANG_PATH = "${pkgs.llvmPackages.libclang.lib}/lib";
}
