# Unmixing

To unmix (recover) the seed phrase you need to supply:

- lang: language of the seed word list to be used
- pin: the pin that was used when the shares were generated with seedmixer
- share files: you will need to supply the locations (on the local machine) of enoungh of the shares to meet the threshold chosen when the shares were generated with seedmixer
- decryption phrase: the decryption phrase that was used when the shares were generated with seedmixer

We will continue the example used in [mixing](./mixing.md). We can supply 2 of the 3 total shares to recover the seed.

```bash
seedmixer unmix --lang eng --pin 1234 --file-path ./secret_enc_share_2_of_3.json --file-path ./secret_enc_share_3_of_3 --decryption-phrase "The Times 03/Jan/2009 Chancellor on brink of second bailout for banks"

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
seedmixer unmix --lang eng --pin 1234 --file-path ./secret_enc_share_2_of_3.json --file-path ./secret_enc_share_3_of_3 --decryption-phrase "The Times 03/Jan/2009 Chancellor on brink of second bailout for banks" --override-file-name lfg
```

The output will be:

```bash
Recovered seed has been stored in lfg_seed.json
```

## Terminal output for recovered seed

If, for some reason, you don't want the recovered seed to be stored in a file but instead just output to the screen you can specify an extra argument to achieve this:

```bash
seedmixer unmix --lang eng --pin 1234 --file-path ./secret_enc_share_2_of_3.json --file-path ./secret_enc_share_3_of_3 --decryption-phrase "The Times 03/Jan/2009 Chancellor on brink of second bailout for banks" --terminal
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