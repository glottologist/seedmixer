# Security Model

This page describes SeedMixer's security guarantees, threat model, and operational recommendations.

## Threat Model

SeedMixer protects against an attacker who obtains **fewer than the threshold number of shares**. The security properties are:

| Layer | Protection | Strength |
|-------|-----------|----------|
| Obfuscation | Shifts word indices with a PIN-derived function | Adds indirection; not secure alone |
| Shamir sharing | Information-theoretic secrecy | Any `t-1` or fewer shares reveal **zero** information about the secret |
| ECIES encryption | Computational secrecy (secp256k1) | Share data is encrypted; an attacker needs the private key even if they have the shares |

### What SeedMixer does NOT protect against

- **Key logger or screen capture** on the machine running SeedMixer
- **Compromise of the threshold number of shares** (by design, `t` shares reconstruct the secret)
- **Loss of the PIN** (the PIN is required for recovery and is not stored)
- **Loss of the decryption key/phrase** when encryption is used
- **Weak PINs** (e.g., `1111`) - SeedMixer does not enforce PIN complexity

## Obfuscation Details

The obfuscation layer shifts each word index using modular exponentiation:

```
obfuscated[i] = seed_index[i] + 2^(pin_digit[i % pin_len]) mod 2049
```

The modulus is `WORDLISTSIZE + 1 = 2049` to keep results within the valid word index range. Deobfuscation reverses the operation with modular subtraction.

> **Note:** Obfuscation alone is weak. A 4-digit PIN has only ~6,500 possible values (digits 1-9). Obfuscation is designed as an additional layer, not a standalone protection.

## Shamir Sharing Details

SeedMixer uses Shamir's Secret Sharing with:

- **Field**: Integers modulo a fixed 1024-bit prime
- **Polynomial degree**: `threshold - 1`
- **Per-word splitting**: Each word index is treated as a separate secret with its own polynomial
- **Reconstruction**: Lagrange interpolation at `x = 0` using Fermat's Little Theorem for modular inverse

The 1024-bit prime ensures the share values are uniformly distributed across a very large space, preventing any leakage from the small (0-2048) word index values.

## Encryption Details

When encryption is enabled, each BigInt share value is:

1. Converted to its little-endian byte representation
2. Encrypted with ECIES using the secp256k1 public key
3. Stored as a `Vec<u8>` in the share file

### Key Generation Modes

| Mode | Key derivation | Storage |
|------|---------------|---------|
| Generated key (`--encrypt`) | Random secp256k1 keypair | Secret key saved to `*_key.json` as hex |
| Passphrase (`--encryption-phrase`) | `SHA-256(phrase)` used as secret key bytes | Nothing saved; user must remember the phrase |

> **Warning:** The passphrase mode derives the key deterministically from the phrase via SHA-256. The security of this mode depends entirely on the entropy of the chosen phrase.

## Secure File Handling

### File Shredding

The `shred` command overwrites files 7 times before deletion using the `file_shred` crate. This mitigates recovery of deleted files from magnetic media.

> **Note:** On SSDs with wear leveling, overwrite-based shredding may not be fully effective. For SSD security, use full-disk encryption.

### TailsOS

The recommended deployment uses TailsOS, which:

- Runs entirely in RAM (no disk writes by default)
- Routes all traffic through Tor (though SeedMixer needs no network)
- Wipes memory on shutdown
- Provides an amnesic environment where generated files exist only for the session

## Operational Security Recommendations

1. **Always run on an air-gapped machine** - SeedMixer needs no network access
2. **Verify immediately** - After creating shares, unmix them to confirm the seed is recoverable
3. **Distribute shares geographically** - Never store the threshold number of shares in one location
4. **Keep fewer than threshold shares online** - For 2-of-3, at most 1 share should be in an accessible digital location
5. **Preserve a copy of SeedMixer** - Keep the exact binary or ISO used to generate shares alongside the shares
6. **Shred local copies** - After distributing shares, securely delete all local copies
7. **Protect the PIN** - The PIN is required for recovery; treat it as a second factor
8. **Protect the decryption key/phrase** - If encryption was used, losing the key means losing access

## Attack Scenarios

### Scenario: Attacker obtains 1 share (of 2-of-3)

**Result:** Zero information gained. Shamir's scheme is information-theoretically secure below the threshold.

### Scenario: Attacker obtains 2 shares (of 2-of-3) but not the PIN

**Result:** Attacker can reconstruct the obfuscated indices. With a brute-force search over valid PINs (up to ~6,500 combinations for a 4-digit PIN), the original seed can be recovered. **Encryption prevents this** by making the share data unreadable without the key.

### Scenario: Attacker obtains encrypted shares and the decryption key but not the PIN

**Result:** Same as above -- the PIN provides minimal additional protection once shares are decryptable.

### Scenario: Attacker obtains the key file but no share files

**Result:** The key alone is useless without the encrypted share data.
