# Building from source

To build from source, one must clone the repo from [github](https://github.com/glottologist/seedmixer.git). Once cloned the seedmixer can be built using either Cargo(Rust) or Nix.

## Cargo(Rust)

You should have Rust and Cargo installed. Navigate to the `rust` directory and run:

```bash
cargo build --release
```

Seed mixer should then be available in `./target/release/seedmixer`.

## Nix (Flakes only)

To build seedmixer just run:

```Nix
nix build .#seedmixer
```

from the repo root. The build target will be at `./result/bin/seedmixer`

> Note: the nix build will run all unit tests and integration tests as part of the build.
