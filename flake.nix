{
  description = "Bundeswetbewerb Informatik";

  inputs =
    {
      nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";

      fenix = {
        url = "github:nix-community/fenix";
        inputs.nixpkgs.follows = "nixpkgs";
      };

    };

  outputs = { self, nixpkgs, fenix, ... }:
    let
      system = "x86_64-linux";
      pkgs = nixpkgs.legacyPackages.${system};
      toolchain = fenix.packages.${system}.minimal.toolchain;
    in
    {
      formatter.${system} = pkgs.nixpkgs-fmt;
      devShells.${system}.default = import ./shell.nix { inherit pkgs; };

      packages.${system}.aufgabe-2 = import ./aufgabe-2.nix { inherit pkgs; inherit toolchain; };
      apps.${system}.aufgabe-2 = {
        type = "app";
        program = "${self.packages.${system}.aufgabe-2}/bin/aufgabe-2";
      };
    };
}
