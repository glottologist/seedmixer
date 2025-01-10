{
  pkgs ? import <nixpkgs> {},
  target ? "x86_64-unknown-linux-gnu",
}: let
  lib = pkgs.lib;
  common = import ../common.nix {inherit pkgs;};
  isCrossCompiling = pkgs.stdenv.hostPlatform != pkgs.stdenv.buildPlatform;
  src = pkgs.lib.cleanSource ./.;
  cargoToml = builtins.fromTOML (builtins.readFile (src + "/Cargo.toml"));
in
  pkgs.rustPlatform.buildRustPackage rec {
    inherit (common) name ver;
    pname = cargoToml.package.name;
    version = cargoToml.package.version;

    cargoLock = {
      lockFile = src + "/Cargo.lock";
    };

    nativeBuildInputs = with pkgs; [
      pkg-config
      m4
      gnum4
      perl
    ];
    inherit src;

    buildInputs = with pkgs;
      [
        openssl
        openssl.dev
        openssh
      ]
      ++ lib.optional isCrossCompiling pkgs.gcc
      ++ lib.optionals stdenv.isDarwin [libiconv Security SystemConfiguration];

    cargoSha256 = pkgs.lib.fakeSha256;

    cargoBuildOptions = ["--release" "--target" target];

    checkPhase = ''
      cargo test
    '';

    meta = with pkgs.stdenv.lib; {
      inherit (common) maintainers homepage description licenses;
    };
  }
