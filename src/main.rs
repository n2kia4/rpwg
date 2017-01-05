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
    let mut n = 1;
    vec.write(LOWERCASE).unwrap();
    if ! matches.is_present("0") {
        vec.write(NUMBER).unwrap();
        n += 1;
    }
    if ! matches.is_present("A") {
        vec.write(UPPERCASE).unwrap();
        n += 1;
    }
    if matches.is_present("S") {
        vec.write(SYMBOL).unwrap();
        n += 1;
    }

    let mut rng = rand::thread_rng();
    if n <= length {
        for _ in 0..qty {
            let mut mtl: Vec<char> = Vec::new();

            mtl.extend_from_slice(&each_choose(&LOWERCASE));

            if ! matches.is_present("0") {
                mtl.extend_from_slice(&each_choose(&NUMBER));
            }

            if ! matches.is_present("A") {
                mtl.extend_from_slice(&each_choose(&UPPERCASE));
            }

            if matches.is_present("S") {
                mtl.extend_from_slice(&each_choose(&SYMBOL));
            }

            let mtl_rand: Vec<char> = (0..length - n)
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

fn each_choose(x: &[u8]) -> Vec<char> {
    let mut rng = rand::thread_rng();
    (0..1).map(|_| *rng.choose(x).unwrap() as char).collect::<Vec<char>>()
}
