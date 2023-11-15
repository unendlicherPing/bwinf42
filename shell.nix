{ pkgs ? import <nixpkgs> { } }:

pkgs.mkShell
{
  buildInputs = [
    pkgs.rustc
    pkgs.cargo
    pkgs.rustfmt
    pkgs.rust-analyzer
    pkgs.clippy
    pkgs.lldb
    pkgs.nil
  ];
}
