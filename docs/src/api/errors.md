# Error Types

SeedMixer uses a typed error hierarchy built with `thiserror`. All errors implement `std::error::Error` and `Display`.

## Error Hierarchy

```
SeedMixerError (top-level)
├── Shamir(ShamirError)
├── Shred(ShredError)
├── File(FileError)
├── Collection(CollectionError)
├── Storage(StorageError)
├── Ascii(AsciiError)
├── CommandLineArgs(CommandLineArgsError)
├── SeedError(SeedPhraseError)
├── ObfuscationError(ObfuscationSeedError)
├── IncorrectNumberOfSharesGenerated
├── NotEnoughThresholdSharesProvided
├── TooManyThresholdSharesProvided
└── IncorrectNumberOfSharesProvided(usize, usize)
```

## `SeedMixerError`

The top-level error type returned by the `main` function and the `SeedMixing` trait methods.

| Variant | Cause |
|---------|-------|
| `Shamir(ShamirError)` | Shamir sharing/reconstruction failure |
| `Shred(ShredError)` | File shredding failure |
| `File(FileError)` | File I/O or parse error |
| `Collection(CollectionError)` | Error collecting seeds or shares from input |
| `Storage(StorageError)` | Error writing shares or seeds to output |
| `Ascii(AsciiError)` | Error generating ASCII art header |
| `CommandLineArgs(CommandLineArgsError)` | Invalid CLI arguments |
| `SeedError(SeedPhraseError)` | Invalid seed phrase |
| `ObfuscationError(ObfuscationSeedError)` | Obfuscation/deobfuscation failure |
| `IncorrectNumberOfSharesGenerated` | Internal: share count doesn't match expected |
| `NotEnoughThresholdSharesProvided` | Fewer shares provided than the threshold |
| `TooManyThresholdSharesProvided` | More shares provided than the threshold |
| `IncorrectNumberOfSharesProvided(t, n)` | Threshold >= total shares |

---

## `SeedPhraseError`

Errors related to seed phrase validation.

| Variant | Message |
|---------|---------|
| `SeedPhraseLengthIsIncorrect(usize)` | Seeds must be 12, 16, or 24 words |
| `SeedPhraseIndexIsOutsideTheBoundsOfTheWordList` | Index not in 1..=2048 |
| `CannotReadContentsOfSeedPhraseFileFromPath` | File read failure |
| `CannotReadWordFromSeedPhraseFile` | Single word parse failure |
| `UnableToDetermineTerminalInput(io::Error)` | stdin read failure |

---

## `ObfuscationSeedError`

Errors during PIN-based obfuscation.

| Variant | Message |
|---------|---------|
| `PinLengthIsIncorrect` | PIN length is not a factor of seed length |
| `InvalidPinIndex` | Internal: PIN index lookup failed |

---

## `ShamirError`

Errors during Shamir sharing operations.

| Variant | Message |
|---------|---------|
| `UnableToParseBigInt(ParseBigIntError)` | Failed to parse the 1024-bit prime |
| `UnableToConvertBigUintToBigInt` | BigUint to BigInt conversion failure |

---

## `CryptError`

Errors during encryption and decryption.

| Variant | Message |
|---------|---------|
| `UnableToEncryptDecrypt(String)` | ECIES operation failed |
| `NoPublicKeyAvailableToEncrypt` | Missing public key |
| `UnableToGetSecretKeyFromBytes(secp256k1::Error)` | Invalid secret key bytes |
| `UnableToGetSecretKeyFromBytesSlice(TryFromSliceError)` | Byte slice conversion failure |
| `UnableToConvertKeyFromHex(hex::FromHexError)` | Hex decoding failure |

---

## `CommandLineArgsError`

Errors during CLI argument parsing.

| Variant | Message |
|---------|---------|
| `PinIsNotAValidNumber(ParseIntError)` | PIN is not a valid integer |
| `PinShouldNotContainZero` | PIN contains a zero digit |
| `FilePathIsNotValid` | Specified file does not exist |

---

## `CollectionError`

Errors during input collection.

| Variant | Message |
|---------|---------|
| `Cryptography(CryptError)` | Decryption failure during share collection |
| `Storage(StorageError)` | I/O error during collection |
| `UnableToReadSeedFromFile(PathBuf)` | Cannot read seed file |
| `UnableToReadWriteFile(io::Error)` | File I/O error |
| `UnableToParseFile(serde_json::Error)` | JSON parse error |
| `UnableToCollectSharesFromFiles` | Duplicate share indices or missing shares |
| `NotEnoughSharesProvided(usize)` | Zero shares provided |

---

## `StorageError`

Errors during output storage.

| Variant | Message |
|---------|---------|
| `UnableToReadWriteFile(io::Error)` | File I/O error |
| `Cryptography(CryptError)` | Encryption failure during share storage |

---

## `FileError`

General file operation errors.

| Variant | Message |
|---------|---------|
| `UnableToReadWriteFile(io::Error)` | File I/O error |
| `UnableToParseFile(serde_json::Error)` | JSON parse error |
| `Cryptography(CryptError)` | Encryption/decryption error |
| `UnableToCollectSharesFromFiles` | Share collection failure |

---

## `ShredError`

Errors during secure file deletion.

| Variant | Message |
|---------|---------|
| `UnableToShredFile(String)` | `file_shred` operation failed |

---

## `AsciiError`

Errors during ASCII art generation.

| Variant | Message |
|---------|---------|
| `UnableToLoadAsciiArtImageFromMemory(ImageError)` | Failed to load embedded logo image |
