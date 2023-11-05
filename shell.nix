{ pkgs ? import <nixpkgs> {} }:

pkgs.mkShell {
  buildInputs = with pkgs; [
    ravedude
    pkgsCross.avr.buildPackages.gcc
    avrdude
  ];
}
# {
#   inputs = {
#     fenix.url = "github:nix-community/fenix";
#     ravedude.url = "github:Rahix/avr-hal?dir=ravedude";
#     nixpkgs.url = "nixpkgs/nixos-unstable";
#   };
#   outputs = {
#     self,
#     nixpkgs,
#     ravedude,
#     fenix,
#   }: let
#     system = "x86_64-linux";
#     pkgs = import nixpkgs {inherit system;};
#   in {
#     devShells.${system}.default = pkgs.mkShell {
#       buildInputs = [
#         pkgs.pkgsCross.avr.buildPackages.gcc
#         pkgs.avrdude
#         ravedude.defaultPackage.${system}
#         (fenix.packages.${system}.fromToolchainFile {
#           file = ./rust-toolchain.toml;
#           sha256 = "sha256-kI+vy5ThOmIdokk5Xtg1I7MyG1xzihcfI0T+hrAgsjA=";
#         })
#       ];
#     };
#   };
# }
