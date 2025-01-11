# SeedMixer ![SeedMixer Logo](./docs/src/assets/Mixer-med.png)

SeedMixer is a command line based Mnemonic (Seed) splitter and encrypter. It is designed to be run offline (i.e. on an air-gapped host).

SeedMixer will obfuscate the original BIP-39 seed phrase and create Shamir shares (N of M) of the phrase and then, optionally, encrypt each share to allow secure storage.

## Usage

Please see [documentation](https://seedmixer.glottologist.co.uk) for usage.

# Security Recommendations

To use seedmixer securely, we urge you to follow the following recommendations:

- Use Tails image OR the binaries on an airgapped machine.
- Shred all local files once shares have been safely moved.
- ALWAYS recover the seed IMMEDIATELY after creating shares to confirm that the original seed can be recovered.
- ALWAYS keep a copy of seedmixer (iso/binary, that was originally used to generate the shares) in a safe an easily accesible place. NEVER assume you can download seedmixer after the initial generation (github outages are not uncommon).
- DO NOT store the shares all together in the same place.
- Never keep the threshold number of shares in online places. I.e. for a 2 of 3 share, 2 of the shares should really be in offline (usb stick in a safe) locations.
- Optionally think about encrypting the usb sticks that you use to transfer the shares from the machine used to generate the shares.

# Buy me a coffee

This software is free to use, however, if you find it useful and want to give something back then coffees are greatly received.

<a href="https://www.buymeacoffee.com/glottologist"><img src="https://img.buymeacoffee.com/button-api/?text=Buy me a coffee&emoji=&slug=glottologist&button_colour=5F7FFF&font_colour=ffffff&font_family=Lato&outline_colour=000000&coffee_colour=FFDD00" /></a>

## Crypto coffee

| Chain   | Address |
| -------- | ------- |
| Ethereum | 0x31A9431e27760628bEbB3f19a09f5ea5366b54A3 |
| Solana   | 7SUpcwHZ4EoTFPFoys5DUEVmWA31uZpeeUVcthdah9hk       |
| Cosmos   | cosmos1ksy92rpu0u5sh663n36n9mqceyzhzzms6radt5         |

# Disclaimer

This software (the “Software”) is provided on an “AS IS” and “AS AVAILABLE” basis without warranties of any kind. The authors, maintainers, contributors, and affiliates (“We”) disclaim any and all responsibility or liability for the completeness, accuracy, reliability, suitability, or availability of the Software, including any tools or features such as bip-39 mnemonic seed generation or Shamir Secret Sharing (seed splitting).

By using this Software, you expressly acknowledge and agree that:

## No Warranty

We do not warrant or guarantee the operation of the Software or that it will be uninterrupted, timely, secure, or error-free. Any use of the Software is at your own risk.

## Loss of Seed Phrase

You are solely responsible for securely storing and backing up any generated or split seed phrases, mnemonics, and cryptographic keys. This includes not verifiying the seed phrase is recoverable at point of usage. We do not accept liability for any damage, loss of data, or financial loss arising from the loss or mismanagement of seed phrases.

## Loss of Funds

Any use of this Software for cryptocurrency storage or management is done at your own discretion and risk. We will not be held responsible for any direct, indirect, consequential, or incidental loss or damages that may occur, including but not limited to loss of funds due to forgotten or misplaced seed phrases.

## Use at Your Own Risk

You are advised to take all necessary precautions and apply best practices, including but not limited to secure backups, offline storage, and personal due diligence before and after using the Software.

## No Liability for Damages

Under no circumstances shall We be liable for any direct, indirect, special, incidental, or consequential damages, whether in contract, tort, or otherwise, arising from the use of or inability to use the Software.

By installing, accessing, or otherwise using the Software, you acknowledge that you have read, understood, and agree to be bound by this Disclaimer. If you do not agree, do not install, access, or use the Software.

![](https://komarev.com/ghpvc/?username=glottologist&style=flat-square&label=Views)
![](https://badges.pufler.dev/visits/glottologist/seedmixer?color=black&logo=github&style=flat-square)
![Last updated](https://img.shields.io/github/last-commit/glottologist/seedmixer/master?label=Last%20updated&style=flat)
![](https://github.com/glottologist/seedmixer/commits)
