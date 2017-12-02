#[macro_use]
extern crate clap;
extern crate rand;

use std::io::Write;
use clap::{Arg, App};
use rand::Rng;

const LOWERCASE: &'static [u8] = b"abcdefghijklmnopqrstuvwxyz";
const UPPERCASE: &'static [u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ";
const NUMBER: &'static [u8] = b"0123456789";
const SYMBOL: &'static [u8] = b"!\"#$%&'()-=^~\\|@`[]{};:+*,./_<>?";

struct Args {
    length: usize,
    quantity: usize,
    lower_c: usize,
    upper_c: usize,
    number_c: usize,
    symbol_c: usize,
    upper: bool,
    number: bool,
    symbol: bool,
}

fn main() {
    let args = parse_args();
    let count = args.lower_c + args.upper_c + args.number_c + args.symbol_c;
    if count > args.length {
        println!("Please specify length more than {}.", count);
    } else {
        generate_pw(&args, count);
    }
}

fn parse_args() -> Args {
    let matches = App::new("rpwg")
        .version(crate_version!())
        .author(crate_authors!())
        .about("Random password generator")
        .arg(Arg::from_usage("-l --length [length] 'Specify the length'").default_value("8"))
        .arg(Arg::from_usage("-q --quantity [quantity] 'Specify the quantity'").default_value("10"))
        .arg(Arg::from_usage("-c --lower_count [lower_count] 'Lowercase count to include'").default_value("1"))
        .arg(Arg::from_usage("-u --upper_count [upper_count] 'Uppercase count to include'").default_value("1"))
        .arg(Arg::from_usage("-n --number_count [number_count] 'Number count to include'").default_value("1"))
        .arg(Arg::from_usage("-s --symbol_count [symbol_count] 'Special symbol count to include'").default_value("1"))
        .arg(Arg::from_usage("-A 'Don't include capital letters'"))
        .arg(Arg::from_usage("-0 'Don't include numbers'"))
        .arg(Arg::from_usage("-S 'Don't include special symbols'"))
        .get_matches();

    Args {
        length: value_t!(matches, "length", usize).unwrap(),
        quantity: value_t!(matches, "quantity", usize).unwrap(),
        upper: matches.is_present("A"),
        number: matches.is_present("0"),
        symbol: matches.is_present("S"),
        lower_c: value_t!(matches, "lower_count", usize).unwrap(),
        upper_c: match matches.is_present("A") {
            true => 0,
            false => value_t!(matches, "upper_count", usize).unwrap(),
        },
        number_c: match matches.is_present("0") {
            true => 0,
            false => value_t!(matches, "number_count", usize).unwrap(),
        },
        symbol_c: match matches.is_present("S") {
            true => 0,
            false => value_t!(matches, "symbol_count", usize).unwrap(),
        },
    }
}

fn generate_pw(args: &Args, count: usize) {
    let mut rng = rand::thread_rng();
    for _ in 0.. args.quantity {
        let mut mtl: Vec<char> = Vec::new();
        mtl.extend_from_slice(&each_choose(args.lower_c, &LOWERCASE));
        mtl.extend_from_slice(&each_choose(args.upper_c, &UPPERCASE));
        mtl.extend_from_slice(&each_choose(args.number_c, &NUMBER));
        mtl.extend_from_slice(&each_choose(args.symbol_c, &SYMBOL));
        mtl.extend_from_slice(&each_choose(args.length - count, &create_element(args)));
        rng.shuffle(&mut mtl);
        let pw: String = mtl.into_iter().collect();
        println!("{}", pw);
    }
}

fn each_choose(count: usize, charset: &[u8]) -> Vec<char> {
    let mut rng = rand::thread_rng();
    (0..count).map(|_| *rng.choose(charset).unwrap() as char).collect::<Vec<char>>()
}

fn create_element(args: &Args) -> Vec<u8> {
    let mut vec: Vec<_> = Vec::new();
    vec.write(LOWERCASE).unwrap();
    if args.upper == false {
        vec.write(UPPERCASE).unwrap();
    }
    if args.number == false {
        vec.write(NUMBER).unwrap();
    }
    if args.symbol == false {
        vec.write(SYMBOL).unwrap();
    }
    return vec;
}
