# passgen

## Installation

Install the tool directly from the GitHub repository using Cargo:

```bash
cargo install --git https://github.com/neotheprogramist/passgen
```

## Basic Usage

To generate a password, simply run `passgen` in your terminal. By default, it will return a password containing 4 digits, 4 special characters, and will have a total length of 16 characters. If you want a personalized password, you can use the options described below.

## Command-Line Arguments

- `-l, --length [default: 16]`: Specifies the total length of the password.
- `-d, --digit-num [default: 4]`: Specifies the number of digits in the password.
- `-s, --special-char-num [default: 4]`: Specifies the number of special characters in the password.

## Examples

Generate a password with a specific length, number of digits, and number of special characters:

```bash
passgen -l 25 -d 6 -s 6
```

Alternatively, using the long-form options:

```bash
passgen --length 25 --digit-num 6 --special-char-num 6
```

---
