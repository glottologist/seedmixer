# System Architecture

SeedMixer is a single-binary CLI application written in Rust. This page describes the module structure, data flow, and key design decisions.

## High-Level Data Flow

### Mix (splitting a seed)

```
 Seed Words ──► WordList lookup ──► Seed (indices)
                                        │
                                        ▼
                              PIN + PowerOfTwoShift
                                        │
                                        ▼
                               ObfuscatedSeed (indices)
                                        │
                                        ▼
                              Shamir.generate_shares()
                              (per index, 1024-bit prime)
                                        │
                                        ▼
                              HashMap<BigInt, SecretShare>
                                        │
                              ┌─────────┴─────────┐
                              ▼                   ▼
                        PlainShareStorer    EncryptedShareStorer
                              │                   │
                              ▼                   ▼
                        JSON files          ECIES-encrypted
                                            JSON files + key
```

### Unmix (recovering a seed)

```
 Share Files ──► FileShareCollector / EncryptedFileShareCollector
                              │
                              ▼
                 HashMap<BigInt, SecretShare>
                              │
                              ▼
                 Shamir.reconstruct_secret()
                 (Lagrange interpolation per index)
                              │
                              ▼
                    ObfuscatedSeed (indices)
                              │
                              ▼
                    PIN + PowerOfTwoUnshift
                              │
                              ▼
                    Seed (original indices)
                              │
                              ▼
                    WordList reverse lookup
                              │
                    ┌─────────┴─────────┐
                    ▼                   ▼
             FileSeedStorer      TerminalSeedStorer
```

## Module Structure

```
rust/src/
├── bin/seedmixer.rs    # CLI entry point, command dispatch
├── lib.rs              # Public module re-exports
├── cli/                # Clap argument parsing, PIN validation
├── seed/               # Seed struct, TerminalSeed, validation
├── words/              # BIP-39 word lists (10 languages, 2048 words each)
├── obfuscation/        # PIN-based index shifting (Obfuscator/Deobfuscator traits)
├── shamir/             # Shamir's Secret Sharing (generate/reconstruct)
├── crypt/              # ECIES encryption (secp256k1 keypair management)
├── mix/                # SeedMixer: orchestrates obfuscation + Shamir
├── collection/         # SeedCollector / ShareCollector traits + impls
│   ├── seed.rs         # FileSeedCollector, TerminalSeedCollector
│   └── share.rs        # FileShareCollector, EncryptedFileShareCollector
├── storage/            # SeedStorer / ShareStorer traits + impls
│   ├── seed.rs         # FileSeedStorer, TerminalSeedStorer
│   └── share.rs        # PlainShareStorer, EncryptedShareStorer
├── shred/              # Secure file deletion (7-pass overwrite)
├── ascii/              # Terminal ASCII art header generation
├── errors/             # Typed error hierarchy (thiserror)
└── data/               # Embedded assets (word lists, logo)
```

## Key Design Decisions

### Three-Layer Security Model

SeedMixer applies security layers incrementally:

1. **Obfuscation** (always applied) - PIN-based modular shifting of word indices. Not cryptographically secure alone, but adds indirection so that share data doesn't directly reveal word positions.

2. **Shamir's Secret Sharing** (always applied) - Information-theoretically secure splitting. Any fewer than `t` shares reveals zero information about the secret.

3. **ECIES Encryption** (optional) - Elliptic curve encryption of share data within each file. Adds defense-in-depth if an attacker obtains shares.

### Trait-Based Abstractions

The system uses trait objects (`Box<dyn T>`) for I/O operations, enabling:

- **`SeedCollector`** - File or terminal input for seed phrases
- **`ShareCollector`** - Plain or encrypted share file reading
- **`SeedStorer`** - File or terminal output for recovered seeds
- **`ShareStorer`** - Plain or encrypted share file writing
- **`Shifter`** - Forward and reverse obfuscation operations
- **`Obfuscator` / `Deobfuscator`** - Seed obfuscation pipeline

### 1024-bit Prime Modulus

Shamir operations use a fixed 1024-bit prime for all modular arithmetic. This provides a large enough field that share values don't leak information about the relatively small (0-2048) word indices.

### Embedded Word Lists

All 10 BIP-39 word lists are compiled into the binary via `include_str!()`. This ensures the tool works fully offline with no external file dependencies.

### JSON Share Format

Shares are serialized as JSON with BigInt values encoded as sign + magnitude arrays by `serde`. This format is human-inspectable and portable across platforms.

## Dependencies

| Crate | Purpose |
|-------|---------|
| `clap` | CLI argument parsing with derive macros |
| `num-bigint` | Arbitrary precision integers for Shamir arithmetic |
| `num-modular` | Modular arithmetic for obfuscation shifting |
| `secp256k1` | Elliptic curve key generation |
| `ecies` | ECIES encryption/decryption |
| `sha2` | SHA-256 hashing for passphrase-derived keys |
| `serde` / `serde_json` | Serialization of shares and seeds |
| `file_shred` | Secure file deletion |
| `thiserror` | Typed error derivation |
| `proptest` | Property-based testing |
| `artem` / `figlet-rs` | ASCII art terminal header |
