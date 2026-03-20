# Contributing

Guidelines for contributing to SeedMixer.

## Prerequisites

- **Rust** (stable, 2021 edition)
- **Cargo** (comes with Rust)
- **Nix** (optional, for reproducible builds and dev environment)

## Development Environment

### With Nix (recommended)

```bash
# Enter the dev shell
nix develop

# Or build directly
nix build .#seedmixer
```

The Nix dev environment includes pre-commit hooks for `cargo check`, `clippy`, and `rustfmt`.

### Without Nix

```bash
cd rust
cargo build
cargo test
cargo clippy
cargo fmt --check
```

## Project Structure

```
seedmixer/
├── rust/                 # Rust crate
│   ├── Cargo.toml
│   ├── src/
│   │   ├── bin/          # CLI entry point
│   │   ├── data/         # Embedded word lists and images
│   │   └── ...           # Library modules
│   └── tests/            # Integration tests
├── docs/                 # mdBook documentation (this site)
│   ├── book.toml
│   └── src/
├── flake.nix             # Nix flake configuration
└── common.nix            # Shared Nix definitions
```

## Testing

SeedMixer uses extensive property-based testing with `proptest`. Many test suites run 1000-5000 cases.

```bash
cd rust

# Run all tests
cargo test

# Run tests for a specific module
cargo test --lib shamir

# Run with output visible
cargo test -- --nocapture
```

### Test Philosophy

- **Property-based tests** (`proptest`) are the primary test tool. They verify invariants across thousands of random inputs.
- **Roundtrip tests** are critical: `unmix(mix(seed)) == seed` must hold for all valid inputs.
- **Standalone tests** are used for I/O operations, error paths, and edge cases.

## Code Quality

All code must pass:

```bash
# No warnings
cargo clippy -- -D warnings

# Formatted
cargo fmt --check

# All tests pass
cargo test
```

## Adding a New Language

To add a new BIP-39 word list:

1. Add the word list file to `rust/src/data/words/<lang_code>/words.txt` (2048 words, one per line)
2. Add the language variant to the `Language` enum in `rust/src/words/mod.rs`
3. Add the `include_str!()` constant in the `WordList` impl
4. Add the language code mapping in `get_language()`
5. Add the language to `get_associated_word_list()`
6. Update `docs/src/supportedlanguages.md`
7. Add tests verifying the word list has exactly 2048 entries

## Documentation

Documentation is built with [mdBook](https://rust-lang.github.io/mdBook/).

```bash
cd docs

# Install mdBook (if not using Nix)
cargo install mdbook

# Build
mdbook build

# Serve locally with hot reload
mdbook serve --open
```

All documentation source is in `docs/src/`. Update `docs/src/SUMMARY.md` when adding new pages.

## License

SeedMixer is licensed under CC BY-NC-ND 4.0. Contributions are accepted under the same license.
