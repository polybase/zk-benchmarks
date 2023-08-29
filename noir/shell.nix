{
    pkgs ? import <nixpkgs> {},
    mkShell ? pkgs.mkShell,
    callPackage ? pkgs.callPackage,
    fetchFromGitHub ? pkgs.fetchFromGitHub,
}: let
    barretenberg = fetchFromGitHub {
        owner = "AztecProtocol";
        repo = "barretenberg";
        rev = "fdd46f77531a6fcc9d9b24a698c56590d54d487e"; # same rev as in https://github.com/noir-lang/noir/blob/13df8d4fbc0104d3bbfdfacaf3afbb47e2eef4b7/flake.lock#L17C17-L17C57
        sha256 = "sha256-w7yMeYp50KrlTn23TTKfYmLOQL4uIgw0wSX67v2tvvc=";
    };
    libbarretenberg = callPackage "${barretenberg}/barretenberg.nix" {};
in mkShell {
    packages = [
        libbarretenberg
        pkgs.libiconv
        pkgs.llvmPackages.openmp
        pkgs.darwin.apple_sdk.frameworks.Security
        pkgs.pkg-config
    ];
}