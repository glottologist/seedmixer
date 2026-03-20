# Command Reference

SeedMixer provides five subcommands. This page documents every flag and argument.

## `seedmixer mix`

Obfuscates a BIP-39 seed phrase with a PIN, splits it into Shamir shares, and optionally encrypts each share.

```bash
seedmixer mix [OPTIONS] --lang <LANG> --pin <PIN> --shares <SHARES> --threshold <THRESHOLD>
```

| Flag | Short | Required | Description |
|------|-------|----------|-------------|
| `--lang` | `-l` | Yes | Language code for the BIP-39 word list (see [Supported Languages](../supportedlanguages.md)) |
| `--pin` | `-p` | Yes | Obfuscation PIN. Digits 1-9 only (no zeros). Length must be a factor of the seed length |
| `--shares` | `-s` | Yes | Total number of shares to generate |
| `--threshold` | `-t` | Yes | Minimum shares needed to reconstruct the seed. Must be less than `--shares` |
| `--file-path` | `-f` | No | Path to a file containing seed words (one per line). If omitted, prompts for terminal input |
| `--encrypt` | `-e` | No | Encrypt shares with a randomly generated secp256k1 key pair |
| `--encryption-phrase` | | No | Encrypt shares with a key derived from this passphrase (SHA-256 hash) |
| `--override-file-name` | `-o` | No | Custom prefix for output share files (default: `secret`) |

**Valid threshold/shares combinations:** 2 of 3, 3 of 4, 4 of 5.

### Output files

| Mode | Share files | Key file |
|------|-----------|----------|
| No encryption | `secret_share_{i}_of_{n}.json` | None |
| `--encrypt` | `secret_enc_share_{i}_of_{n}.json` | `secret_enc_share_key.json` |
| `--encryption-phrase` | `secret_enc_share_{i}_of_{n}.json` | None (key derived from phrase) |

### Examples

```bash
# Non-encrypted, 2-of-3, English word list, terminal input
seedmixer mix --lang eng --pin 1234 --threshold 2 --shares 3

# Non-encrypted, seed from file
seedmixer mix --lang eng --pin 1234 --threshold 2 --shares 3 --file-path ./my_seed.txt

# Encrypted with generated key
seedmixer mix --lang eng --pin 1234 --threshold 3 --shares 5 --encrypt

# Encrypted with passphrase
seedmixer mix --lang eng --pin 1234 --threshold 2 --shares 3 \
  --encryption-phrase "my memorable phrase"

# Custom file prefix
seedmixer mix --lang eng --pin 1234 --threshold 2 --shares 3 --override-file-name wallet1
```

---

## `seedmixer unmix`

Reconstructs the original seed phrase from threshold shares, optionally decrypting them first.

```bash
seedmixer unmix [OPTIONS] --lang <LANG> --pin <PIN> --file-path <PATH>...
```

| Flag | Short | Required | Description |
|------|-------|----------|-------------|
| `--file-path` | `-f` | Yes | Path to a share file. Repeat for each share provided |
| `--lang` | `-l` | Yes | Language code for the BIP-39 word list |
| `--pin` | `-p` | Yes | The same PIN used during mixing |
| `--decryption-key` | `-d` | No | Hex-encoded secret key from the generated key file. Conflicts with `--decryption-phrase` |
| `--decryption-phrase` | | No | The passphrase used during mixing. Conflicts with `--decryption-key` |
| `--terminal` | `-t` | No | Output seed to terminal instead of a file |
| `--override-file-name` | `-o` | No | Custom prefix for the output seed file (default: `seed`) |

> **Note:** You must supply exactly the threshold number of shares (not more, not fewer).

### Examples

```bash
# Non-encrypted recovery
seedmixer unmix --lang eng --pin 1234 \
  --file-path ./secret_share_1_of_3.json \
  --file-path ./secret_share_3_of_3.json

# Encrypted recovery with generated key
seedmixer unmix --lang eng --pin 1234 \
  --file-path ./secret_enc_share_1_of_3.json \
  --file-path ./secret_enc_share_2_of_3.json \
  --decryption-key "263f1e06c441c4964eba4770dea9c608a9886ebb45607a01435337c33d24a253"

# Encrypted recovery with passphrase
seedmixer unmix --lang eng --pin 1234 \
  --file-path ./secret_enc_share_1_of_3.json \
  --file-path ./secret_enc_share_2_of_3.json \
  --decryption-phrase "my memorable phrase"

# Output to terminal instead of file
seedmixer unmix --lang eng --pin 1234 --terminal \
  --file-path ./secret_share_1_of_3.json \
  --file-path ./secret_share_2_of_3.json
```

---

## `seedmixer shred`

Securely deletes files by overwriting them 7 times before removal.

```bash
seedmixer shred --file-path <PATH>...
```

| Flag | Short | Required | Description |
|------|-------|----------|-------------|
| `--file-path` | `-f` | Yes | Path to a file to shred. Repeat for each file |

### Example

```bash
seedmixer shred \
  --file-path ./secret_share_1_of_3.json \
  --file-path ./secret_share_2_of_3.json \
  --file-path ./secret_share_3_of_3.json
```

---

## `seedmixer check-word-list`

Looks up a word by its position in a BIP-39 word list.

```bash
seedmixer check-word-list --lang <LANG> --position <POS>
```

| Flag | Short | Required | Description |
|------|-------|----------|-------------|
| `--lang` | `-l` | Yes | Language code |
| `--position` | `-p` | Yes | Word position (0-2047) |

Alias: `seedmixer chk`

### Example

```bash
seedmixer check-word-list --lang eng --position 10
# Output: Found access from English wordlist
```

---

## `seedmixer check-word-index`

Looks up the index of a word in a BIP-39 word list.

```bash
seedmixer check-word-index --lang <LANG> --word <WORD>
```

| Flag | Short | Required | Description |
|------|-------|----------|-------------|
| `--lang` | `-l` | Yes | Language code |
| `--word` | `-w` | Yes | The word to look up |

Alias: `seedmixer chi`

### Example

```bash
seedmixer check-word-index --lang eng --word access
# Output: Found 10 from English wordlist
```
