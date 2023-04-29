use std::fs;
use gnu_coreutils::common;

extern crate colored;

use colored::*;

fn read_to_string(filename: &str) -> String {
    let mut is_error = false;

    let bytes = fs::read(filename).unwrap_or_else(|e| {
        is_error = true;
        return format!("{}", e).red().as_bytes().to_vec();
    });

    let mut string = String::from_utf8_lossy(&bytes).to_string();
    
    if is_error { string = string.red().to_string() }

    string
}

fn main() {
    let args = common::parse_args();
    if args.is_empty() { println!("{}", "Please supply a file name.".red()); }
    else { println!("{}", read_to_string(&args[0])); }    
}