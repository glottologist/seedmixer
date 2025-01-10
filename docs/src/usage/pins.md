# Pins

All methods of mixing include the [obfuscation](../methods/obfuscation.md) step. This will shift the seed word indices according to the given pin.

Each digit of the pin should be between 1 and 9 (inclusive) and the length of the pin should be a factor of the length of the seed. The pin **should not contain zeros**.

Valid pin lengths for different seed sizes are:

| Seed word length | Pin lengths     |
| ---------------- | --------------- |
| 24 words         | 4, 6, 8, 12, 24 |
| 16 words         | 4, 6, 8, 16     |
| 12 words         | 4, 6, 12        |

Example pins (not to actually be used as pins):

| Seed word length | Pins                                                           |
| ---------------- | -------------------------------------------------------------- |
| 24 words         | 1234, 123456, 12345678, 123456123456, 123456123456123456123456 |
| 16 words         | 1234, 123456, 12345678, 1234567812345678                       |
| 12 words         | 1234, 123456, 123456123456                                     |
