# Binaries

The binaries files for seedmixer can be found on the [releases](https://github.com/glottologist/seedmixer/releases) page of the code repository.

Download the appropriate binary to the your local machine and run from a terminal. Optionally, you can confirm the hash of the file by comparing it against the sha256 file that accompanies the binary file.

```bash
shasum -a 256 seedmixer_<architecture>_<version>
```

This should match the hash that is in the `seedmixer_<architecture>_<version>.sha256` file.

### Linux/MacOS/WSL

Open a terminal and use `curl`:

```bash
curl -L -o seedmixer https://github.com/glottologist/seedmixer/releases/download/v1.0.0/seedmixer_x86_64-unknown-linux-gnu_v1.0.0
```

You will need to make the binary executable:

```bash
sudo chmod +x seedmixer
```

### Windows (Native)

## Usage

Once the binary is downloaded, open a terminal window and you will have access to the seedmixer by running the commands in the [usage](../usage/usage.md) documentation.

> Note (Windows users): When using seedmixer, replace any `seedmixer` commands with `seedmixer.exe`.
