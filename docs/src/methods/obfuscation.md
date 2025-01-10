# Obfuscation

The obfuscation stage of seedmixer takes the indices of the seed phrase and a given [pin](../usage) and uses the digits of the pin to shift the seed indices in a deterministic manner in order to 'scramble' the seed words.

Obfuscation isn't a secure encryption process, it just adds another layer of indirection.

To see a worked example, we shall take a 12 word seed phrase:

```bash
route vendor find blossom angry document either ill cricket surge seek often
```

The indices of these words in the BIP-39 English word list are:

```bash
1508, 1937, 693, 192, 71, 515, 568, 903, 411, 1746, 1560, 1229
```

If we take a pin 9246, then the indices are split into groups of 4. All the first digits in the groups are shifted by 2 to the power of the first pin digit modulo 2048. All the second digits are shifter by 2 to the power of the second digit modulo 2048, etc.

| index | pin digit | obfuscation           |
| ----- | --------- | --------------------- |
| 1508  | 9         | 1508 + (2^9 mod 2048) |
| 1937  | 2         | 1937 + (2^2 mod 2048) |
| 693   | 4         | 693 + (2^4 mod 2048)  |
| 192   | 6         | 192 + (2^6 mod 2048)  |
| 71    | 9         | 71 + (2^9 mod 2048)   |
| 515   | 2         | 515 + (2^2 mod 2048)  |
| 568   | 4         | 568 + (2^4 mod 2048)  |
| 903   | 6         | 903 + (2^6 mod 2048)  |
| 411   | 9         | 411 + (2^9 mod 2048)  |
| 1746  | 2         | 1746 + (2^2 mod 2048) |
| 1560  | 4         | 1560 + (2^4 mod 2048) |
| 1229  | 6         | 1229 + (2^6 mod 2048) |

The result is:

```bash
2020, 1941, 709, 256, 583, 519, 584, 967, 923, 1750, 1576, 1293
```

The corresponding words from the word list for the obfuscated seed would be:

```
wish verify flavor cactus employ domain empower jump inflict suspect shallow payment
```

It is the obfuscated seed that is then passed on for Shamir splitting
