{
  description = "Flake for SeedMixer";

  inputs = {
    #nixpkgs.url = "github:cachix/devenv-nixpkgs/rolling";
    nixpkgs.url = "github:NixOS/nixpkgs/release-24.11";
    fenix.url = "github:nix-community/fenix";
    devenv.url = "github:cachix/devenv";
    devenv.inputs.nixpkgs.follows = "nixpkgs";
    flake-parts.url = "github:hercules-ci/flake-parts";
    flake-utils.url = "github:numtide/flake-utils";
    nix2container.url = "github:nlewo/nix2container";
    nix2container.inputs.nixpkgs.follows = "nixpkgs";
    mk-shell-bin.url = "github:rrbutani/nix-mk-shell-bin";
  };

  nixConfig = {
    extra-substituters = [
      "https://tweag-jupyter.cachix.org"
      "https://devenv.cachix.org"
    ];
    extra-trusted-public-keys = [
      "tweag-jupyter.cachix.org-1:UtNH4Zs6hVUFpFBTLaA4ejYavPo5EFFqgd7G7FxGW9g="
      "devenv.cachix.org-1:w1cLUi8dv3hnoSPGAuibQv+f9TZLr6cv/Hm9XgU50cw="
    ];
  };

  outputs = inputs @ {
    flake-parts,
    flake-utils,
    nixpkgs,
    ...
  }:
    flake-parts.lib.mkFlake {inherit inputs;} {
      imports = [
        inputs.devenv.flakeModule
      ];

      systems = inputs.nixpkgs.lib.systems.flakeExposed;

      perSystem = {
        config,
        self',
        inputs',
        pkgs,
        system,
        ...
      }: rec {
        packages = rec {
          rustcross = {target}:
            pkgs.callPackage ./rust/default.nix {
              inherit pkgs;
              inherit target;
            };
          seedmixer = pkgs.callPackage ./rust/default.nix {inherit pkgs;};
          default = self'.packages.seedmixer;
        };
        apps = {
          seedmixerApp = flake-utils.lib.mkApp {drv = self'.packages.${system}.seedmixer;};
        };

        devenv.shells.default = devenv.shells.seedmixer;
        devenv.shells.seedmixer = {
          name = "SeedMixer shell for Rust";
          env.GREET = "devenv for the Rust flavour of SeedMixer";
          env.PKG_CONFIG_PATH = "${pkgs.openssl.dev}/lib/pkgconfig";
          packages = with pkgs; [
            git
            mdbook
            mdbook-i18n-helpers
            mdbook-mermaid
            openssh
            openssl
            pkg-config
            perl
          ];
          enterShell = ''
            cargo install cargo-nextest
            cargo install cargo-tree
            cargo install cargo-udeps
            git --version
            nix --version
            rustc --version
            cargo --version
            mdbook --version
          '';
          languages = {
            rust.enable = true;
            rust.channel = "nightly";
            nix.enable = true;
          };
          scripts = {
            ntest.exec = ''
              cargo nextest run
            '';
            tree.exec = ''
              cargo tree
            '';
            watch.exec = ''
              cargo watch -c -q -w ./src -x build
            '';
          };
          dotenv.enable = true;
          difftastic.enable = true;
          pre-commit = {
            hooks = {
              alejandra.enable = true;
              commitizen.enable = true;
              cargo-check.enable = true;
              clippy.enable = true;
              rustfmt.enable = true;
              nil.enable = true;
            };
            settings.rust.cargoManifestPath = "./rust/Cargo.toml";
          };
        };
      };
    };
}
