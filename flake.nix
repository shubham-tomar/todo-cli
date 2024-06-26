{
  description = "nix-rust-template";

  nixConfig.bash-prompt = "[nix]λ ";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";

    naersk = {
      url = "github:nmattia/naersk";
      inputs.nixpkgs.follows = "nixpkgs";
    };

    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };

    flake-utils = {
      url = "github:numtide/flake-utils";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { self, flake-utils, naersk, nixpkgs, rust-overlay }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [
          rust-overlay.overlay
          naersk.overlay
        ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };
        rust = pkgs.rust-bin.stable.latest.default;
        naersk-lib = pkgs.naersk.override {
          cargo = pkgs.rust-bin.nightly.latest.cargo;
          rustc = rust;
        };
        rust-dev = rust.override {
          extensions = [
            "clippy-preview"
            "rust-src"
            "rustc-dev"
            "rustfmt-preview"
          ];
        };
      in rec {
        # `nix build`
        packages.todo-cli = naersk-lib.buildPackage {
          pname = "todo-cli";
          root = ./.;
        };
        defaultPackage = packages.todo-cli;

        # `nix run` or `nix run .#app`
        apps.app = flake-utils.lib.mkApp {
          drv = packages.todo-cli;
        };
        defaultApp = apps.app;

        # `nix run .#watch`
        apps.watch = flake-utils.lib.mkApp {
          drv = pkgs.writeShellApplication {
            name = "watch";
            runtimeInputs = [
              pkgs.cargo-watch
              pkgs.gcc
              rust
            ];
            text = ''
              cargo-watch -w "./src/" -x "run"
            '';
          };
        };

        # `nix develop`
        devShell = pkgs.mkShell {
          buildInputs = [
            pkgs.cargo-edit
            pkgs.cargo-watch
            pkgs.rust-analyzer
          ];
          nativeBuildInputs = [ rust-dev ];
        };
      }
    );
}