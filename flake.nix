{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs";
    utils.url = "github:numtide/flake-utils";
    naersk.url = "github:nix-community/naersk";
  };

  outputs = { self, nixpkgs, utils, naersk }:
    utils.lib.eachDefaultSystem (system:
    let
      pkgs = nixpkgs.legacyPackages."${system}";
      naersk-lib = naersk.lib."${system}";
    in rec 
    {
      packages.jade = naersk-lib.buildPackage {
        pname = "jade";
        root = ./.;
      };
      
      packages.default = packages.jade;

      apps.jade = utils.lib.mkApp {
        drv = packages.jade;
      };
      
      apps.default = apps.jade;

      devShells.default = pkgs.mkShell {
        nativeBuildInputs = with pkgs; [
          rustc
          cargo
          rustfmt
          cargo-audit
          clippy
        ];
      };
    });
}

