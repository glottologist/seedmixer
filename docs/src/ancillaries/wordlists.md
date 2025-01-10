# Word Lists

Each seed phrase is made up of words selected from the same BIP-39 word list of 2048 words. These word lists are [specified](https://github.com/bitcoin/bips/tree/master/bip-0039) in multiple languages.

Seedmixer provides two methods to quickly check words and indexes for the word list in each language.

## Finding the word for a given index

To check the word for a given ordered index in the BIP-39 list, one can use the `check-word-list` method:

```bash
seedmixer check-word-list --lang <lang> --position <pos>
```

where:

- lang: is the language shortname for one of the [supported languages](../supportedlanguages.md)
- pos: the index of the word in the word list from 1 to 2048

For example, if we want to find the word at index 10 in the word list for English:

```bash
seedmixer check-word-list --lang eng --position 10
```

which will return `Found access from English wordlist`.

To see the word at the same postition on the Spanish word list:

```bash
seedmixer check-word-list --lang spa --position 10
```

which will return `Found abuso from Spanish wordlist`.

## Finding the index for a given word

To check the ordered index of a word in the BIP-39 list, one can use the `check-word-index` method:

```bash
seedmixer check-word-index --lang <lang> --word <word>
```

where:

- lang: is the language shortname for one of the [supported languages](../supportedlanguages.md)
- word: the word in the word list

For example, if we want to find the index for `access` in the word list for English:

```bash
seedmixer check-word-index --lang eng --word access
```

which will return `Found 10 from English wordlist`.

To see the index of the word `abuso` on the Spanish word list:

```bash
seedmixer check-word-index --lang spa --word abuso
```

which will return `Found 10 from Spanish wordlist`.
