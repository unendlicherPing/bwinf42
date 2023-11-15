{
  description = "Bundeswetbewerb Informatik";

  inputs =
    {
      nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    };

  outputs = { self, nixpkgs, ... }:
    let
      system = "x86_64-linux";
      pkgs = nixpkgs.legacyPackages.${system};
    in
    {
      formatter.${system} = pkgs.nixpkgs-fmt;
      devShells.${system}.default = import ./shell.nix { inherit pkgs; };

      packages.${system}.aufgabe-2 = import ./aufgabe-2.nix { inherit pkgs; };
      apps.${system}.aufgabe-2 = {
        type = "app";
        program = self.outputs.packages.${system}.aufgabe-2;
      };
    };
}
