#[macro_use]
extern crate clap;
extern crate rand;

use std::io::Write;
use clap::{Arg, App};
use rand::Rng;

const LOWERCASE: &'static [u8] = b"abcdefghijklmnopqrstuvwxyz";
const UPPERCASE: &'static [u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ";
const DIGITS: &'static [u8] = b"0123456789";
const SYMBOL: &'static [u8] = b"!\"#$%&'()-=^~\\|@`[]{};:+*,./_<>?";

struct Args {
    upper: bool,
    digits: bool,
    symbol: bool,
    length: usize,
    num_pw: usize,
    lower_c: usize,
    upper_c: usize,
    digits_c: usize,
    symbol_c: usize,
}

fn main() {
    let args = parse_args();
    let count = args.lower_c + args.upper_c + args.digits_c + args.symbol_c;
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
        .arg(Arg::from_usage("-U 'Don't include uppercase letters'"))
        .arg(Arg::from_usage("-D 'Don't include digits'"))
        .arg(Arg::from_usage("-S 'Don't include symbols'"))
        .arg(Arg::from_usage("-l --length [length] 'Length of the password'").default_value("8"))
        .arg(Arg::from_usage("-n --num_pw [num_pw] 'Number of passwords'").default_value("10"))
        .arg(Arg::from_usage("-c --lower_count [lower_count] 'Minimum number of lowercase letters to include in password'").default_value("1"))
        .arg(Arg::from_usage("-u --upper_count [upper_count] 'Minimum number of uppercase letters to include in password'").default_value("1"))
        .arg(Arg::from_usage("-d --digits_count [digits_count] 'Minimum number of digits to include in password'").default_value("1"))
        .arg(Arg::from_usage("-s --symbol_count [symbol_count] 'Minimum number of symbols to include in password'").default_value("1"))
        .get_matches();

    Args {
        upper: matches.is_present("U"),
        digits: matches.is_present("D"),
        symbol: matches.is_present("S"),
        length: value_t!(matches, "length", usize).unwrap(),
        num_pw: value_t!(matches, "num_pw", usize).unwrap(),
        lower_c: value_t!(matches, "lower_count", usize).unwrap(),
        upper_c: match matches.is_present("U") {
            true => 0,
            false => value_t!(matches, "upper_count", usize).unwrap(),
        },
        digits_c: match matches.is_present("D") {
            true => 0,
            false => value_t!(matches, "digits_count", usize).unwrap(),
        },
        symbol_c: match matches.is_present("S") {
            true => 0,
            false => value_t!(matches, "symbol_count", usize).unwrap(),
        },
    }
}

fn generate_pw(args: &Args, count: usize) {
    let mut rng = rand::thread_rng();
    for _ in 0.. args.num_pw {
        let mut mtl: Vec<char> = Vec::new();
        mtl.extend_from_slice(&each_choose(args.lower_c, &LOWERCASE));
        mtl.extend_from_slice(&each_choose(args.upper_c, &UPPERCASE));
        mtl.extend_from_slice(&each_choose(args.digits_c, &DIGITS));
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
    if args.digits == false {
        vec.write(DIGITS).unwrap();
    }
    if args.symbol == false {
        vec.write(SYMBOL).unwrap();
    }
    return vec;
}
