# rpwg

Random password generator.

## Installation

You can use the `cargo install` command:

    $ cargo install rpwg

or a classic build and run:

```
git clone https://github.com/n2kia4/rpwg.git
cd rpwg
cargo build --release && cargo run --release
```

## Options

You can check by typing `rpwg -h`:


```
USAGE:
    rpwg [FLAGS] [OPTIONS]

FLAGS:
    -0               Don't include numbers
    -A               Don't include capital letters
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -l, --length <length>    Specify the length (default: 8)
```
