# File shredding

The file shredding utility uses the [file shred crate](https://crates.io/crates/file_shred) to re-encrypt a specified file seven times before deleting the file.

To shred files, just call the shred command with the file-path argument of each file.

```bash
seedmixer shred --file-path ./file1.json --file-path ./file2.json
```


