# Mixing

To mix (without encrypting the shares) you will need to supply:

- lang: language of the seed word list to be used
- pin: the pin, whose length is a factor of the seed word length
- shares: total shares that the seed should be split into.
- threshold: the number of shares required to recover the seed phrase.

> The valid threshold/shares are: 2 of 3, 3 of 4, or 4 of 5. The choice of this is down to preference and need. 5 total shares requires more places to distribute to but offers better redundancy than 3 total shares.

As an example, the command for a 2 of 3 shares of the English word list could be:

```bash
seedmixer mix --lang eng --pin 1234 --threshold 2 --shares 3
```

Seedmixer will ask for the seed words to be entered:

```bash
Please enter a list of words separated by spaces.
```

> Security: Do not run the seedmixer on a remote machine, it should be on a local machine.

For this example we will use a 12 word phrase which we enter into the terminal:

```bash
eye guilt market language fall target engine wealth believe puzzle surround point
```

This will generate 3 share files:

```bash
Share stored secret_share_2_of_3.json
Share stored secret_share_1_of_3.json
Share stored secret_share_3_of_3.json
```

Each share will have the following structure:

```json
{
  "index": [1, [1]],
  "threshold": [1, [2]],
  "total": [1, [3]],
  "shares": [
    [
      1,
      [
        3907216828, 55218690, 1700888717, 856572667, 790502508, 2401527878,
        3255086926, 3934230631, 2428670545, 561384297, 378030278, 1639322054,
        506534203, 2663039889, 3348008826, 1907486980
      ]
    ],
    [
      1,
      [
        4041749841, 2547894823, 3859475869, 2195869928, 450389550, 1688949311,
        3965918807, 706016198, 2553048678, 1249312350, 4034844566, 4262996688,
        863988542, 881882373, 1608995180, 2180868176
      ]
    ],
    [
      1,
      [
        1359225059, 4181181110, 1717174512, 2125947024, 1238213107, 3743910844,
        3190936707, 3261940107, 2568661646, 1047021912, 1043801537, 3639906157,
        1865774981, 3671617226, 1165634285, 1755450596
      ]
    ],
    [
      1,
      [
        3423723478, 4209918997, 3550252394, 176630731, 2059807916, 715033109,
        2194064145, 2594832661, 3369132833, 1811781012, 2694783494, 1979421907,
        2260229213, 1126178111, 3428765980, 2217636743
      ]
    ],
    [
      1,
      [
        1049871540, 936634232, 1872806735, 2592740916, 1780730692, 2729328913,
        1000879784, 2637785901, 3038651378, 4148235585, 1856082740, 2595377507,
        688985741, 491946673, 3606886018, 832124243
      ]
    ],
    [
      1,
      [
        386725569, 3106083561, 3589795591, 2989414655, 4072952721, 2534805072,
        1729649352, 3356597378, 537153837, 2327071673, 2116107827, 732733449,
        538456738, 1270392486, 4080044465, 925886770
      ]
    ],
    [
      1,
      [
        518108489, 3002219994, 1867856988, 725053884, 3626884347, 2368248281,
        368146484, 1735232784, 2252311523, 2725165662, 428783700, 2746831239,
        449774402, 3998878326, 972924165, 1469230420
      ]
    ],
    [
      1,
      [
        2008963071, 3775837030, 4168231317, 3016568310, 1371335559, 3102217230,
        1887726416, 77130299, 4086970320, 885053593, 1174379250, 1403617248,
        3535526762, 3508755075, 3738920032, 61123959
      ]
    ],
    [
      1,
      [
        3555170461, 4058034113, 126588865, 693103831, 1339108562, 2618171782,
        148769271, 3274770097, 1106214716, 2312892354, 830756308, 1335797257,
        1727232875, 4010960328, 3819737210, 531367510
      ]
    ],
    [
      1,
      [
        844318088, 3416476047, 749430106, 921697294, 2529423047, 1713027185,
        522716926, 3141446208, 739393894, 4033600181, 224750664, 429603723,
        1726859527, 765017162, 1689575810, 2111217747
      ]
    ],
    [
      1,
      [
        2356347152, 2139425682, 353430235, 2658159841, 3866766202, 759498481,
        3662912678, 1076390464, 1511837737, 709339311, 2852816387, 414102063,
        3723672781, 2031386014, 4095688584, 540743756
      ]
    ],
    [
      1,
      [
        3384550249, 759916013, 764602365, 2113512013, 2230715634, 3952823134,
        3100759278, 1332805764, 2708347093, 235202943, 127558396, 1809930164,
        ##        3514022026, 3354110835, 727939864, 2048432412
      ]
    ]
  ]
}
```

These files can now be distributed as you see fit.

> WARNING: Do not store these shares all in the same place and consider [shredding](../../anclillaries/shredding.md) the copies locally (i.e. on the machine on which you used seedmixer and generated the shares with) if you are using binaries rather than Tails.

## Overriding share file name

If you have multiple seed phrases to create shares from (or just want a memorable name) you can override the share file prefix to be more meaningful:

```bash
seedmixer mix --lang eng --pin 1234 --threshold 2 --shares 3 --override-file-name lfg
```

This will generate the shares as:

```bash
Share stored lfg_share_2_of_3.json
Share stored lfg_share_1_of_3.json
Share stored lfg_share_3_of_3.json
```
