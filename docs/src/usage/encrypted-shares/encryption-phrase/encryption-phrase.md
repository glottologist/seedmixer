# Encrypted Shares (encryption phrase)

This mode of mixing will include:

- [Obfuscation](../methods/obfuscation.md)
- [Shamir sharing](../methods/shamir.md)
- [Encryption](../methods/encryption.md)

## Mixing (hiding)

[Mixing](./mixing.md) will obfuscate the seed words and then split them into the specified number of shares and finally, encrypt the share data with an encryption key derived from the encryption phrase.

## Unmixing (recovery)

[Unmixing](./unmixing.md) will take the threshold number of shares, decrypt the share data and then reconstruct the original seed.
