rpwg
====
[![Crates.io](https://img.shields.io/crates/v/rpwg.svg)](https://crates.io/crates/rpwg)

Random password generator.

## Installation

You can use the `cargo install` command:

    $ cargo install rpwg

or a classic build and run:

```bash
$ git clone https://github.com/n2kia4/rpwg.git
$ cd rpwg
$ cargo build --release && cargo run --release
```

## Options

You can check by typing `rpwg -h`:


```
USAGE:
    rpwg [FLAGS] [OPTIONS]

FLAGS:
    -U               Don't include uppercase letters
    -D               Don't include digits
    -S               Don't include symbols
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -l, --length <length>                Length of the password [default: 8]
    -n, --num_pw <num_pw>                Number of passwords [default: 10]
    -c, --lower_count <lower_count>      Minimum number of lowercase letters to include in password [default: 1]
    -u, --upper_count <upper_count>      Minimum number of uppercase letters to include in password [default: 1]
    -d, --digits_count <digits_count>    Minimum number of digits to include in password [default: 1]
    -s, --symbol_count <symbol_count>    Minimum number of symbols to include in password [default: 1]
```
