{
  description = "Bundeswetbewerb Informatik";

  inputs =
  {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
  };

  outputs = { self, nixpkgs, ... }: 
    let
      system = "aarch64-linux";
      pkgs = nixpkgs.legacyPackages.${system};
      in
    {
      devShells.${system}.default = (import ./shell.nix { inherit pkgs; });
    };
}
