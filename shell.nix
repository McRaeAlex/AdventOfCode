{ pkgs ? import <nixpkgs> {} }:

pkgs.mkShell {
    packages = with pkgs; [
        figlet

        ghc
        ocaml
        python3

        # rust dependencies
        cargo
        rustc
        rustfmt
        clippy
        rust-analyzer
    ];

    shellHook = ''
        figlet "Advent of Code!"
    '';
}
