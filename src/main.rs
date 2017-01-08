#[macro_use]
extern crate clap;
extern crate rand;

use std::io::Write;
use clap::{Arg, App};
use rand::Rng;

const NUMBER: &'static [u8] = b"0123456789";
const UPPERCASE: &'static [u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ";
const LOWERCASE: &'static [u8] = b"abcdefghijklmnopqrstuvwxyz";
const SYMBOL: &'static [u8] = b"!\"#$%&'()-=^~\\|@`[]{};:+*,./_<>?";

struct Args {
    length: usize,
    quantity: usize,
    number: bool,
    upper: bool,
    symbol: bool,
}

fn main() {
    let args = parse_args();
    let mut vec: Vec<u8> = Vec::new();
    let mut n = 1;

    vec.write(LOWERCASE).unwrap();
    if args.number == false {
        vec.write(NUMBER).unwrap();
        n += 1;
    }
    if args.upper == false {
        vec.write(UPPERCASE).unwrap();
        n += 1;
    }
    if args.symbol == true {
        vec.write(SYMBOL).unwrap();
        n += 1;
    }

    let mut rng = rand::thread_rng();
    if n <= args.length {
        for _ in 0.. args.quantity {
            let mut mtl: Vec<char> = Vec::new();

            mtl.extend_from_slice(&each_choose(&LOWERCASE));

            if args.number == false {
                mtl.extend_from_slice(&each_choose(&NUMBER));
            }

            if args.upper == false {
                mtl.extend_from_slice(&each_choose(&UPPERCASE));
            }

            if args.symbol == true {
                mtl.extend_from_slice(&each_choose(&SYMBOL));
            }

            let mtl_rand: Vec<char> = (0..args.length - n)
                .map(|_| *rng.choose(&vec).unwrap() as char)
                .collect();
            mtl.extend_from_slice(&mtl_rand);

            rng.shuffle(&mut mtl);
            let pw: String = mtl.into_iter().collect();
            println!("{}", pw);
        }
    } else {
        println!("Please specify length more than {}.", n);
    }
}

fn parse_args() -> Args {
    let matches = App::new("rpwg")
        .version(crate_version!())
        .author(crate_authors!())
        .about("Random password generator")
        .arg(Arg::from_usage("-l --length [length] 'Specify the length'").default_value("8"))
        .arg(Arg::from_usage("-q --quantity [quantity] 'Specify the quantity'").default_value("10"))
        .arg(Arg::from_usage("-0 'Don't include numbers'"))
        .arg(Arg::from_usage("-A 'Don't include capital letters'"))
        .arg(Arg::from_usage("-S 'Include special symbols'"))
        .get_matches();

    Args {
        length: value_t!(matches, "length", usize).unwrap(),
        quantity: value_t!(matches, "quantity", usize).unwrap(),
        number: matches.is_present("0"),
        upper: matches.is_present("A"),
        symbol: matches.is_present("S"),
    }
}

fn each_choose(x: &[u8]) -> Vec<char> {
    let mut rng = rand::thread_rng();
    (0..1).map(|_| *rng.choose(x).unwrap() as char).collect::<Vec<char>>()
}
