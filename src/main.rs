#![deny(clippy::all)]
use std::env;
use std::fs;

pub mod compile;
pub mod lex;
pub mod parse;

use compile::*;

fn main() {
    let filenames: Vec<String> = env::args().collect();
    let file =
        fs::read_to_string(filenames[1].clone()).expect("Something went wrong reading the file");
    let result = file.compile();
    println!("{}", result);
}
