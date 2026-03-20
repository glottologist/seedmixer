# Module Overview

SeedMixer's Rust library (`seedmixer`) is organized into 12 public modules. This page provides a map of each module's responsibility and its key exports.

## Module Map

| Module | Responsibility | Key Exports |
|--------|---------------|-------------|
| `cli` | CLI argument parsing and validation | `Cli`, `Command`, `MixArgs`, `UnMixArgs`, `ShredArgs` |
| `seed` | Seed phrase representation and input | `Seed`, `TerminalSeed` |
| `words` | BIP-39 word list management (10 languages) | `WordList`, `Language`, `WORDLISTSIZE` |
| `obfuscation` | PIN-based seed index shifting | `Pin`, `ObfuscatedSeed`, `Shifter`, `Obfuscator`, `Deobfuscator` |
| `shamir` | Shamir's Secret Sharing (split/reconstruct) | `Shamir` |
| `crypt` | ECIES encryption with secp256k1 | `Crypt`, `EncSettings` |
| `mix` | Orchestrates obfuscation + Shamir splitting | `SeedMixer`, `SeedMixing`, `SecretShare` |
| `collection` | Input collection from files/terminal | `SeedCollector`, `ShareCollector`, `get_seed_collector`, `get_share_collector` |
| `storage` | Output storage to files/terminal | `SeedStorer`, `ShareStorer`, `EncryptedSecretShare`, `get_seed_storer`, `get_share_storer` |
| `shred` | Secure file deletion | `Shred`, `ShredFile` |
| `ascii` | Terminal ASCII art header | `generate_terminal_header` |
| `errors` | Typed error hierarchy | `SeedMixerError`, `ShamirError`, `CryptError`, etc. |

## Dependency Graph

```
bin/seedmixer.rs
    │
    ├── cli          (argument parsing)
    ├── ascii        (terminal header)
    ├── words        (word list lookup)
    ├── collection   (seed/share input)
    │   ├── seed     → seed, words, errors
    │   └── share    → crypt, mix, storage, errors
    ├── mix          (orchestration)
    │   ├── obfuscation → seed, words, errors
    │   └── shamir      → errors
    ├── storage      (seed/share output)
    │   ├── seed     → seed, words, errors
    │   └── share    → crypt, mix, errors
    └── shred        (file deletion)
```

## Using as a Library

SeedMixer can be used as a Rust library by adding it as a dependency. All public modules are re-exported from `lib.rs`:

```rust
use seedmixer::{
    seed::Seed,
    words::WordList,
    obfuscation::{Pin, Obfuscator, PowerOfTwoShift},
    shamir::Shamir,
    mix::{SeedMixer, SeedMixing},
    crypt::{Crypt, EncSettings},
    errors::SeedMixerError,
};
```

### Minimal Example: Mix and Unmix

```rust
use seedmixer::{
    seed::Seed,
    obfuscation::Pin,
    mix::{SeedMixer, SeedMixing},
};

// Create a seed from word indices (12 words)
let indices = vec![100, 200, 300, 400, 500, 600, 700, 800, 900, 1000, 1100, 1200];
let seed = Seed::new(indices).unwrap();

// Define a PIN
let pin = Pin(vec![1, 2, 3, 4]);

// Mix into 3 shares with threshold 2
let shares = SeedMixer::mix(seed, &pin, 3, 2).unwrap();

// Take any 2 shares to reconstruct
let recovery: std::collections::HashMap<_, _> =
    shares.into_iter().take(2).collect();
let recovered = SeedMixer::unmix(recovery, &pin).unwrap();
```
