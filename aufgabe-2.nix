{ pkgs, toolchain }:

(pkgs.makeRustPlatform {
  cargo = toolchain;
  rustc = toolchain;
}).buildRustPackage {
  name = "aufgabe-2";
  pname = "aufgabe-2";
  src = ./.;

  cargoLock.lockFile = ./Cargo.lock;
}
