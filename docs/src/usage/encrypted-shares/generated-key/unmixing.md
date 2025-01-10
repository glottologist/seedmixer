# Unmixing

To unmix (recover) the seed phrase you need to supply:

- lang: language of the seed word list to be used
- pin: the pin that was used when the shares were generated with seedmixer
- share files: you will need to supply the locations (on the local machine) of enoungh of the shares to meet the threshold chosen when the shares were generated with seedmixer
- decryption key: the decryption key that was generated when the shares were generated with seedmixer

We will continue the example used in [mixing](./mixing.md). We can supply 2 of the 3 total shares to recover the seed.

```bash
seedmixer unmix --lang eng --pin 1234 --file-path ./secret_enc_share_2_of_3.json --file-path ./secret_enc_share_3_of_3 --decryption-key "263f1e06c441c4964eba4770dea9c608a9886ebb45607a01435337c33d24a253"

```

The output will be:

```bash
Recovered seed has been stored in seed.json
```

The contents of the seed file with be:

```json
[
  "eye",
  "guilt",
  "market",
  "language",
  "fall",
  "target",
  "engine",
  "wealth",
  "believe",
  "puzzle",
  "surround",
  "point"
]
```

## Overriding share file name

If you want the seed file to have an alternate name you can override that like so:

```bash
seedmixer unmix --lang eng --pin 1234 --file-path ./secret_enc_share_2_of_3.json --file-path ./secret_enc_share_3_of_3 --decryption-key "263f1e06c441c4964eba4770dea9c608a9886ebb45607a01435337c33d24a253" --override-file-name lfg
```

The output will be:

```bash
Recovered seed has been stored in lfg_seed.json
```

## Terminal output for recovered seed

If, for some reason, you don't want the recovered seed to be stored in a file but instead just output to the screen you can specify an extra argument to achieve this:

```bash
seedmixer unmix --lang eng --pin 1234 --file-path ./secret_enc_share_2_of_3.json --file-path ./secret_enc_share_3_of_3 --decryption-key "263f1e06c441c4964eba4770dea9c608a9886ebb45607a01435337c33d24a253" --terminal
```

No seed file will be created and the terminal output will be:

```bash
Seed is:
eye
guilt
market
language
fall
target
engine
wealth
believe
puzzle
surround
point

```
