# Troubleshooting

Common issues and their solutions when using SeedMixer.

## Mix Errors

### "Pin should not contain any zeros"

**Cause:** The PIN contains a `0` digit.

**Fix:** Use only digits 1-9 in your PIN. For example, `1234` is valid but `1024` is not.

---

### "Number of seed entries are incorrect"

**Cause:** The seed phrase does not have 12, 16, or 24 words.

**Fix:** Verify your seed phrase word count. BIP-39 only supports these three lengths.

---

### "Seed phase indices can only fall between 1 and 2048"

**Cause:** One or more words in your seed phrase are not in the BIP-39 word list for the specified language.

**Fix:**
- Check spelling of each word
- Verify you're using the correct `--lang` flag
- Use `seedmixer check-word-index --lang <lang> --word <word>` to verify individual words

---

### "Incorrect number of shares provided: threshold X, total Y"

**Cause:** The threshold value is greater than or equal to the total shares.

**Fix:** Threshold must be strictly less than total shares. Valid examples: 2 of 3, 3 of 5, 4 of 7.

---

### "File path is not valid"

**Cause:** The `--file-path` argument points to a file that doesn't exist.

**Fix:** Verify the file path. Use an absolute path or ensure the relative path is correct from your current working directory.

## Unmix Errors

### "Not enough threshold shares provided"

**Cause:** Fewer share files were provided than the threshold encoded in the shares.

**Fix:** Provide exactly the threshold number of share files. Check the share file JSON to see the `threshold` field.

---

### "Too many threshold shares provided"

**Cause:** More share files were provided than the threshold.

**Fix:** Provide exactly the threshold number of shares, not more.

---

### Incorrect seed recovered

**Possible causes:**
1. **Wrong PIN** - The PIN must be identical to the one used during mixing
2. **Wrong language** - The `--lang` flag must match the language used during mixing
3. **Corrupted share files** - A share file may have been modified or truncated
4. **Wrong decryption key/phrase** - If encryption was used, the key must match exactly

**Fix:** Double-check all parameters match those used during the original `mix` operation.

---

### "Unable to encrypt/decrypt data"

**Cause:** The decryption key or phrase doesn't match the one used during encryption.

**Fix:**
- For generated keys: use the exact hex string from the `*_key.json` file
- For passphrases: the phrase must match character-for-character (case-sensitive, including spaces)

---

### "Unable to parse file"

**Cause:** A share file is not valid JSON or doesn't match the expected schema.

**Fix:**
- Verify the file hasn't been corrupted or truncated
- Ensure you're pointing to share files, not the key file or seed file
- If using encrypted shares, ensure you're providing encrypted share files (not plain share files) with the appropriate decryption flag

## Build Errors

### OpenSSL link errors

**Cause:** The `openssl` crate requires system OpenSSL headers.

**Fix:** SeedMixer uses the `vendored` feature for OpenSSL, which should compile it from source. If this still fails:

```bash
# Debian/Ubuntu
sudo apt-get install pkg-config libssl-dev

# macOS
brew install openssl

# Nix
nix build .#seedmixer
```

---

### `include_str!` or `include_bytes!` errors

**Cause:** The `data/` directory containing word lists and images is missing.

**Fix:** Ensure you cloned the full repository including the `data/` directory under `rust/src/data/`.

## General Tips

1. **Always verify immediately after mixing** - Run `unmix` right after `mix` to confirm your seed is recoverable before distributing shares.

2. **Keep a copy of the binary** - Store the exact version of SeedMixer used to create your shares. Future versions might change the serialization format.

3. **Check share file contents** - Share files are JSON and can be opened in any text editor. Verify the `threshold` and `total` fields match your expectations.

4. **Use `--terminal` for quick verification** - When testing recovery, use `--terminal` to avoid creating unnecessary seed files on disk.
