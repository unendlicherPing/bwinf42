{ pkgs ? import <nixpkgs> {} }:

pkgs.stdenv.mkDerivation {
	name = "aufgabe-2";
	src = ./.;

	buildInputs = [
		pkgs.rustc
		pkgs.cargo
	];

	buildPhase = ''
		cargo build --release --bin aufgabe-2
	'';

	installPhase = ''
		mkdir -p $out/bin
		cp target/release/aufgabe-2 $out/bin
	'';
}
