# TailsOS

TailsOS is a portable operating system that protects against surveillance and censorship. It provides a live OS environment from which to run the seedmixer operations.

> Warning: do not create or use persistent storage when using Tails and seedmixer unless you shred any related files in the persistent storage location.

## Download Tails with seedmixer

The ISO file for Tails with seedmixer bundled can be found on the [releases](https://github.com/glottologist/seedmixer/releases) page of the code repository.

Download the iso to the your local machine. Optionally, you can confirm the hash of the file by comparing it against the iso.sha256 file that accompanies the tails.iso file.

```bash
shasum -a 256 tails-amd64-<tails_version>-seedmixer-<version>.iso
```

This should match the hash that is in the `tails-amd64-<tails_version>-seedmixer-<version>.iso.sha256` file.

## Burn ISO for memory stick

The iso can be burned using any software that can burn isos to memory sticks but we recommend [balena etcher](https://etcher.balena.io/). It supports Linux, MacOS, and Windows. Alternatives are:

- [Rufus](https://rufus.ie/en/) - (for Windows)
- [Ventoy](https://www.ventoy.net/en/index.html)

Please follow the instuctions to burn to disc.

## Usage

Once burned to disc/stick you can boot tails and follow the initial live cd setup to specify language settings etc. From there, open a terminal window from the `Applications -> Utilities` menu and you will have access to the seedmixer by running the commands in the [usage](../usage/usage.md) documentation.
