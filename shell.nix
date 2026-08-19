{ pkgs ? import <nixpkgs> {} }:

pkgs.mkShell {
  buildInputs = with pkgs; [
    janet
    cmake_3_16
    pkg-config
    gtk3
    libui
    hiredis
  ];
}