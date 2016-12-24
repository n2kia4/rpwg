#[macro_use]

extern crate clap;
extern crate rand;

use std::io::Write;
use clap::{Arg, App};
use rand::Rng;

const NUMBER: &'static [u8] = b"0123456789";
const UPPERCASE: &'static [u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ";
const LOWERCASE: &'static [u8] = b"abcdefghijklmnopqrstuvwxyz";

fn main() {
    let matches = App::new("rpwg")
        .version("v0.1.0")
        .author("Natsuki Tanaka <n2kia4@gmail.com>")
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
        .get_matches();

    let length = value_t!(matches, "length", usize).unwrap_or(8);

    let mut vec: Vec<u8> = Vec::new();
    vec.write(LOWERCASE).unwrap();
    if matches.is_present("0") {
        {}
    } else {
        vec.write(NUMBER).unwrap();
    }
    if matches.is_present("A") {
        {}
    } else {
        vec.write(UPPERCASE).unwrap();
    }

    let mut rng = rand::thread_rng();
    let pw: String = (0..length).map(|_| *rng.choose(&vec).unwrap() as char).collect();

    println!("Generated password: {}", pw);
}
