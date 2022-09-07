{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs";
    utils.url = "github:numtide/flake-utils";
    naersk = {
      url = "github:nix-community/naersk";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = {
    self,
    nixpkgs,
    utils,
    naersk,
    fenix,
    ...
  }:
    utils.lib.eachDefaultSystem (system: let
        pkgs = nixpkgs.legacyPackages."${system}";
        toolchain = with fenix.packages."${system}";
          combine [
            latest.rustc
            latest.cargo
            latest.clippy
            latest.rustfmt
            latest.rust-analyzer
            latest.rust-src
          ];
        naersk-lib = naersk.lib."${system}".override {
          cargo = toolchain;
          rustc = toolchain;
          clippy = toolchain;
          rustfmt = toolchain;
          rust-analyzer = toolchain;
          rust-src = toolchain;
        };
      in rec
      {
        packages.jade = naersk-lib.buildPackage {
          pname = "Jade";
          root = ./.;
        };

        packages.default = packages.jade;

        apps.jade = utils.lib.mkApp {
          drv = packages.jade;
        };

        apps.default = apps.jade;

        devShells.default = pkgs.mkShell {
          nativeBuildInputs = [
            toolchain
          ];
          # For rust-analyzer
          RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";
        };

        formatter = pkgs.alejandra;
      });
}
