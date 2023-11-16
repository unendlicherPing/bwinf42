{ pkgs, toolchain }:

(pkgs.makeRustPlatform {
  cargo = toolchain;
  rustc = toolchain;
}).buildRustPackage {
  name = "bwinf42";
  pname = "bwinf42";
  src = ./.;

  cargoLock.lockFile = ./Cargo.lock;
}
