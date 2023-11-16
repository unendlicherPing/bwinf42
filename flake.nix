{
  description = "Bundeswetbewerb Informatik 42";

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

      packages.${system}.default = import ./default.nix { inherit pkgs; inherit toolchain; };

      apps.${system} = {
        default = {
          type = "app";
          program =
            let
              message = pkgs.writeShellScriptBin "message" ''
                echo "please use nix run .#aufgabe-[1|2] instead"
              '';
            in
            "${message}/bin/message";
        };
        aufgabe-1 = {
          type = "app";
          program = "${self.packages.${system}.default}/bin/aufgabe-1";
        };
        aufgabe-2 = {
          type = "app";
          program = "${self.packages.${system}.default}/bin/aufgabe-2";
        };
      };
    };
}
