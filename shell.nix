{ pkgs ? import <nixpkgs> { } }:

pkgs.mkShell
{
  buildInputs = [
    pkgs.rustup
    pkgs.rust-analyzer
    pkgs.lldb
    pkgs.nil
  ]; 
}