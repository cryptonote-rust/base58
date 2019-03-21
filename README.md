# Base58 Library For CryptoNote Based Crypto Currencies


[![](https://travis-ci.com/cryptonote-rust/base58.svg?branch=master)](https://travis-ci.com/cryptonote-rust/base58)
[![](https://img.shields.io/crates/v/cryptonote-base58.svg)](https://crates.io/crates/cryptonote-base58)
[![codecov](https://codecov.io/gh/cryptonote-rust/base58/branch/master/graph/badge.svg)](https://codecov.io/gh/cryptonote-rust/base58)



# Usage

1. to_base58

```
        let bytes = vec![0x3d, 0xf1, 0x2e, 0x7, 0xad, 0x81, 0xf0, 0x9d, 0xcc, 0x3e, 0x78, 0x7b, 
        0x68, 0xc6, 0xdc, 0x73, 0xea, 0xfd, 0xae, 0xa6, 0x43, 0x33, 0x6d,
         0xfe, 0x62, 0xad, 0xa4, 0xab, 0x3c, 0xc4, 0x9d, 0x67, 0xfc, 0xcb, 0x86, 0x6f, 0xf0, 0xba, 0xc3, 0xfc, 0x4a, 0x51, 0x22, 0xa, 0x47, 0x82, 0xcb, 0x8f, 0xdc, 0x59, 0x5a, 0x56, 0xba, 0x90, 0xf, 0x41, 0xd, 0x28, 0xfe, 0x70, 0xb8, 0x4d, 0x8a, 0x60, 0xc9, 0x5a, 0x22, 0xb5, 0xee];

        let address = to_base58(bytes);
```

2. from_base58
```
        let bytes = from_base58(String::from("BMv8aSohtktbARXdSBrVRkgJhyoNBmxmjHWJaVhFx31gjHRqzxXH5G7DRy6uPdPJhkdrfNpF1K5Di3Cfr78ZuEA3PiaUPG9"));
```