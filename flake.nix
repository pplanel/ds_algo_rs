{
  description = " A Rust library to quickly get the size and line size of your CPU caches";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable-small";
    crane = {
      url = "github:ipetkov/crane";
      inputs.nixpkgs.follows = "nixpkgs";
      inputs.flake-compat.follows = "flake-compat";
      inputs.utils.follows = "utils";
    };
    flake-compat = {
      url = "github:edolstra/flake-compat";
      flake = false;
    };
    gitignore = {
      url = "github:hercules-ci/gitignore.nix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    rust = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
      inputs.flake-utils.follows = "utils";
    };
    utils.url = "github:numtide/flake-utils";
  };

  outputs =
    { self
    , nixpkgs
    , crane
    , gitignore
    , rust
    , utils
    , ...
    }:
    utils.lib.eachSystem (utils.lib.defaultSystems ++ [ "x86_64-linux" ]) (system:
    let
      pkgs = import nixpkgs {
        inherit system;
        overlays = [ gitignore.overlay rust.overlay ];
      };

      toolchain = pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;

      craneLib = (crane.mkLib pkgs).overrideScope' (_: _: {
        cargo = toolchain;
        clippy = toolchain;
        rustc = toolchain;
        rustfmt = toolchain;
      });

      src = pkgs.gitignoreSource ./.;

      cargoArtifacts = craneLib.buildDepsOnly {
        inherit src;
      };
      fmt = craneLib.cargoFmt { inherit cargoArtifacts src; };
      clippy = craneLib.cargoClippy {
        inherit src;
        cargoArtifacts = fmt;
        cargoClippyExtraArgs = "-- --deny warnings";
      };
      crate = craneLib.buildPackage {
        inherit src;
        cargoArtifacts = clippy;
      };
    in
    {
      checks = { inherit fmt clippy; };
      defaultPackage = self.packages."${system}".algors;
      packages.algors = { };

      defaultApp = self.apps."${system}".algors;

      apps.algors = {
        type = "app";
        program = "${self.defaultPackage."${system}"}/bin/algors";
      };

      devShell = with pkgs;
        mkShell {
          name = "cache-size";
          nativeBuildInputs = [
            (rust-bin.fromRustupToolchainFile ./rust-toolchain.toml)
            cargo-edit
            cargo-udeps
            cargo-watch
            rust-analyzer
            rustup

            nixpkgs-fmt
            rnix-lsp

            pre-commit
            pkgs.llvmPackages.bintools
          ];

          buildInputs = [
            websocat
          ];
          shellHook = ''
            pre-commit install --install-hooks
          '';
        };
    });
}
