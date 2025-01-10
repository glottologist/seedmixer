# Share Encryption

Share encryption is done with either a generated key pair or a key pair derived from a phrase.

The keypairs use the [secp256k1](https://crates.io/crates/secp256k1) crate which implements ECDSA and BIP 340 signatures for the SECG elliptic curve group `secp256k1`.

Once the keypair is generated/derived, the encryption is done using the [ecies](https://crates.io/crates/ecies) crate which is the Elliptic Curve Integrated Encryption Scheme for secp256k1.

The encryption doesn't encrypt the whole share file, just the share data within the file.
