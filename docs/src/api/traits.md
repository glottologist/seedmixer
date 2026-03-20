# Traits

SeedMixer uses trait-based abstractions for I/O operations and the obfuscation pipeline. This allows different implementations to be swapped at runtime (e.g., file vs. terminal I/O).

## Core Pipeline Traits

### `Shifter`

**Module:** `seedmixer::obfuscation`

Defines how a single seed digit is shifted by a PIN digit.

```rust
pub trait Shifter {
    fn shift(&self, pin_digit: u16, seed_digit: u16) -> u16;
}
```

**Implementations:**

| Struct | Operation |
|--------|-----------|
| `PowerOfTwoShift` | `seed_digit + 2^pin_digit mod 2049` (forward shift) |
| `PowerOfTwoUnshift` | `seed_digit - 2^pin_digit mod 2049` (reverse shift) |

---

### `Obfuscator`

**Module:** `seedmixer::obfuscation`

Obfuscates a seed using a PIN and a `Shifter` strategy.

```rust
pub trait Obfuscator {
    fn obfuscate(
        &self,
        pin: &Pin,
        shifter: impl Shifter,
    ) -> Result<ObfuscatedSeed, ObfuscationSeedError>;
}
```

**Implemented by:** `Seed`

---

### `Deobfuscator`

**Module:** `seedmixer::obfuscation`

Reverses obfuscation to recover the original seed.

```rust
pub trait Deobfuscator {
    fn deobfuscate(
        &self,
        pin: &Pin,
        shifter: impl Shifter,
    ) -> Result<Seed, ObfuscationSeedError>;
}
```

**Implemented by:** `ObfuscatedSeed`

---

### `SeedMixing`

**Module:** `seedmixer::mix`

The top-level orchestration trait for mixing and unmixing seeds.

```rust
pub trait SeedMixing {
    fn mix(
        seed: Seed,
        pin: &Pin,
        total_shares: usize,
        threshold: usize,
    ) -> Result<HashMap<BigInt, SecretShare>, SeedMixerError>;

    fn unmix(
        shares: HashMap<BigInt, SecretShare>,
        pin: &Pin,
    ) -> Result<Seed, SeedMixerError>;
}
```

**Implemented by:** `SeedMixer`

The `mix` method:
1. Validates that `threshold < total_shares`
2. Obfuscates the seed with `PowerOfTwoShift`
3. Generates Shamir shares for each obfuscated index
4. Returns shares keyed by share index

The `unmix` method:
1. Validates the share count equals the threshold
2. Groups share values by word position
3. Reconstructs each word index via Lagrange interpolation
4. Deobfuscates with `PowerOfTwoUnshift`

---

## I/O Traits

### `SeedCollector`

**Module:** `seedmixer::collection::seed`

Collects a seed phrase from an input source.

```rust
pub trait SeedCollector {
    fn collect(&self) -> Result<Seed, CollectionError>;
}
```

| Implementation | Source |
|---------------|--------|
| `FileSeedCollector` | Reads words from a file (one word per line) |
| `TerminalSeedCollector` | Prompts user for space-separated words on stdin |

**Factory function:** `get_seed_collector(wl, path)` returns the appropriate collector based on whether a file path is provided.

---

### `ShareCollector`

**Module:** `seedmixer::collection::share`

Collects secret shares from file sources.

```rust
pub trait ShareCollector {
    fn collect(&self) -> Result<HashMap<BigInt, SecretShare>, CollectionError>;
}
```

| Implementation | Source |
|---------------|--------|
| `FileShareCollector` | Reads plaintext JSON share files |
| `EncryptedFileShareCollector` | Reads encrypted share files and decrypts with the provided key/phrase |

**Factory function:** `get_share_collector(paths, key, phrase)` returns the appropriate collector.

---

### `SeedStorer`

**Module:** `seedmixer::storage::seed`

Stores a recovered seed to an output destination.

```rust
pub trait SeedStorer {
    fn store(&self, seed: Seed) -> Result<(), StorageError>;
}
```

| Implementation | Destination |
|---------------|-------------|
| `FileSeedStorer` | Writes seed words as JSON to a file |
| `TerminalSeedStorer` | Prints seed words to stdout |

**Factory function:** `get_seed_storer(wl, terminal, override_name)` returns the appropriate storer.

---

### `ShareStorer`

**Module:** `seedmixer::storage::share`

Stores generated shares to files.

```rust
pub trait ShareStorer {
    fn store(&self, shares: HashMap<BigInt, SecretShare>) -> Result<(), StorageError>;
}
```

| Implementation | Format |
|---------------|--------|
| `PlainShareStorer` | Writes shares as plaintext JSON files |
| `EncryptedShareStorer` | Encrypts share data with ECIES, writes encrypted JSON + key file |

**Factory function:** `get_share_storer(override_name, enc_settings)` returns the appropriate storer.

---

### `ShredFile`

**Module:** `seedmixer::shred`

Securely deletes files.

```rust
pub trait ShredFile {
    fn shred(&self, file_paths: Vec<PathBuf>) -> Result<(), ShredError>;
}
```

**Implemented by:** `Shred` (7-pass overwrite by default)
