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

fn main() {
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

    let length = value_t!(matches, "length", usize).unwrap();
    let qty = value_t!(matches, "quantity", usize).unwrap();

    let mut vec: Vec<u8> = Vec::new();
    vec.write(LOWERCASE).unwrap();
    if ! matches.is_present("0") {
        vec.write(NUMBER).unwrap();
    }
    if ! matches.is_present("A") {
        vec.write(UPPERCASE).unwrap();
    }
    if matches.is_present("S") {
        vec.write(SYMBOL).unwrap();
    }

    let mut rng = rand::thread_rng();
    for _ in 0..qty {
        let pw: String = (0..length).map(|_| *rng.choose(&vec).unwrap() as char).collect();
        println!("{}", pw);
    }
}
