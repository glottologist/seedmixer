# Core Types

This page documents the primary data structures in SeedMixer.

## `Seed`

**Module:** `seedmixer::seed`

Represents a validated BIP-39 seed phrase as a vector of word indices.

```rust
pub struct Seed {
    pub indices: Vec<u16>,
    pub length: u16,
}
```

### Construction

```rust
// From word indices (must be 12, 16, or 24 elements; each 1..=2048)
let seed = Seed::new(vec![100, 200, 300, /* ... 12/16/24 total */])?;
```

**Validation rules:**
- Length must be exactly 12, 16, or 24
- Each index must be in the range `1..=2048`

**Errors:** Returns `SeedPhraseError::SeedPhraseLengthIsIncorrect` or `SeedPhraseError::SeedPhraseIndexIsOutsideTheBoundsOfTheWordList`.

### Traits Implemented

| Trait | Notes |
|-------|-------|
| `Debug` | Derived |
| `PartialEq` | Derived |
| `Display` | Formats as `[i1, i2, ..., iN]` |
| `Obfuscator` | Obfuscates indices with a `Shifter` and `Pin` |
| `From<TerminalSeed>` | Converts terminal input to `Seed` |

---

## `Pin`

**Module:** `seedmixer::obfuscation`

A wrapper around a vector of PIN digits.

```rust
pub struct Pin(pub Vec<u16>);
```

Each digit must be in the range `1..=9`. The PIN length must be a factor of the seed length.

PIN validation happens at CLI parse time via the `rationalise_pin` function (in `cli`).

---

## `ObfuscatedSeed`

**Module:** `seedmixer::obfuscation`

The result of applying PIN-based obfuscation to a `Seed`.

```rust
pub struct ObfuscatedSeed {
    pub indices: Vec<u16>,
    pub length: u16,
}
```

### Traits Implemented

| Trait | Notes |
|-------|-------|
| `Debug` | Derived |
| `Deobfuscator` | Reverses obfuscation with a `Shifter` and `Pin` |

---

## `SecretShare`

**Module:** `seedmixer::mix`

Represents one participant's share of the split secret.

```rust
pub struct SecretShare {
    pub index: BigInt,      // Share number (1-based)
    pub threshold: BigInt,  // Minimum shares for reconstruction
    pub total: BigInt,      // Total shares generated
    pub shares: Vec<BigInt>, // One Shamir share value per word index
}
```

Derives `Debug`, `Clone`, `Serialize`, `Deserialize`.

---

## `EncryptedSecretShare`

**Module:** `seedmixer::storage`

An encrypted version of `SecretShare` where each share value is ECIES-encrypted bytes.

```rust
pub struct EncryptedSecretShare {
    pub index: BigInt,
    pub threshold: BigInt,
    pub total: BigInt,
    pub shares: Vec<Vec<u8>>,  // Encrypted byte vectors
}
```

Derives `Debug`, `Clone`, `Serialize`, `Deserialize`.

---

## `Shamir`

**Module:** `seedmixer::shamir`

Manages Shamir's Secret Sharing generation and reconstruction.

```rust
pub struct Shamir {
    total_shares: usize,
    threshold: usize,
}
```

### Methods

| Method | Signature | Description |
|--------|-----------|-------------|
| `new` | `(total_shares: usize, threshold: usize) -> Self` | Creates a new Shamir instance |
| `generate_shares` | `(&self, secret: &BigUint) -> Result<Vec<(BigInt, BigInt)>, ShamirError>` | Generates `total_shares` (x, y) points from a degree `threshold-1` polynomial |
| `reconstruct_secret` | `(&self, shares: &[(BigInt, BigInt)]) -> Result<BigInt, ShamirError>` | Reconstructs the secret from `threshold` or more shares via Lagrange interpolation |

---

## `Crypt`

**Module:** `seedmixer::crypt`

Holds a secp256k1 keypair for ECIES encryption and decryption.

```rust
pub struct Crypt {
    pub pub_key: PublicKey,
    pub secret_key: SecretKey,
}
```

### Construction

| Constructor | Description |
|------------|-------------|
| `Crypt::new(EncSettings::Key(None))` | Random keypair |
| `Crypt::new(EncSettings::Key(Some(hex)))` | From hex-encoded secret key |
| `Crypt::new(EncSettings::Phrase(s))` | Key derived from SHA-256 of `s` |
| `Crypt::from_secret_key_bytes(&[u8; 32])` | From raw 32-byte secret key |
| `Crypt::default()` | Random keypair (via `Key(None)`) |

### Methods

| Method | Signature | Description |
|--------|-----------|-------------|
| `encrypt` | `(&self, data: Vec<u8>) -> Result<Vec<u8>, CryptError>` | ECIES encrypt with the public key |
| `decrypt` | `(&self, data: Vec<u8>) -> Result<Vec<u8>, CryptError>` | ECIES decrypt with the secret key |

---

## `EncSettings`

**Module:** `seedmixer::crypt`

Configuration for how encryption keys are derived.

```rust
pub enum EncSettings {
    Phrase(String),       // Derive key from SHA-256 of the phrase
    Key(Option<String>),  // None = generate random; Some(hex) = use existing
}
```

---

## `WordList`

**Module:** `seedmixer::words`

Manages a BIP-39 word list for a specific language, providing bidirectional lookup between words and indices.

```rust
pub struct WordList {
    pub words: HashMap<u16, String>,     // index → word
    pub inverted: HashMap<String, u16>,  // word → index
    pub lang: Language,
}
```

### Methods

| Method | Signature | Description |
|--------|-----------|-------------|
| `new` | `(language: &str) -> Self` | Creates a word list for the given language code |
| `find` | `(&self, index: u16) -> String` | Returns the word at the given index |
| `get_index` | `(&self, word: &str) -> &u16` | Returns the index of the given word |

---

## `Shred`

**Module:** `seedmixer::shred`

Securely deletes files with configurable overwrite passes.

```rust
pub struct Shred {
    passes: u32,  // Default: 7
}
```

Implements the `ShredFile` trait.
