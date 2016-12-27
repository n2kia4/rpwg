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
        .arg(Arg::with_name("length")
             .short("l")
             .long("length")
             .takes_value(true)
             .help("Specify the length (default: 8)"))
        .arg(Arg::with_name("0")
             .short("0")
             .help("Don't include numbers"))
        .arg(Arg::with_name("A")
             .short("A")
             .help("Don't include capital letters"))
        .arg(Arg::with_name("S")
             .short("S")
             .help("Include special symbols"))
        .get_matches();

    let length = value_t!(matches, "length", usize).unwrap_or(8);

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
    let pw: String = (0..length).map(|_| *rng.choose(&vec).unwrap() as char).collect();

    println!("Generated password: {}", pw);
}
