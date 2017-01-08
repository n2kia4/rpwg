# rpwg

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
    -0               Don't include numbers
    -A               Don't include capital letters
    -S               Don't include special symbols
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -l, --length <length>                Specify the length [default: 8]
    -q, --quantity <quantity>            Specify the quantity [default: 10]
    -n, --number_count <number_count>    Number count to include [default: 1]
    -u, --upper_count <upper_count>      Uppercase count to include [default: 1]
    -c, --lower_count <lower_count>      Lowercase count to include [default: 1]
    -s, --symbol_count <symbol_count>    Special symbol count to include [default: 1]
```
