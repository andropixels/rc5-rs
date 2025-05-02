
# RC5 Block Cipher

## Overview


A minimal RC5 block cipher implementation in Rust.

-What is RC5?





RC5 is a symmetric-key block cipher designed by Ronald Rivest in 1994. It is notable for its simplicity and parameterized design, offering flexibility in word size, number of rounds, and key size.


## 🚀 Installation

To use this crate, add the following to your Cargo.toml:
```
[dependencies]
rc5-block-cipher = "0.1.0"

```

## Quick Start

### Example Usage

```
use rc5_block_cipher::RC5;

let key = b"your_secret_key";
let rounds = 12;
let rc5 = RC5::new(key, rounds);

// Encrypt
let mut data = [0u8; 8];
data.copy_from_slice(b"testdata");
rc5.encrypt_block(&mut data);

// Decrypt
rc5.decrypt_block(&mut data);

```


### Features

Pure Rust implementation

Zero unsafe code

32-bit word size

Configurable rounds

Simple API


### Limitations

This implementation is for educational purposes only and is not recommended for production use.

## Contributing

Contributions are welcome! Feel free to submit a pull request or open an issue.

## License

This project is licensed under the MIT License.
