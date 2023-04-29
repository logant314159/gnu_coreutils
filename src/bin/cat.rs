use std::fs;
use gnu_coreutils::common;

extern crate colored;

use colored::*;

/// Returns a string containing the contents of the file, or a red error string if an error occurred.
fn read_to_string(filename: &str) -> String {
    let mut is_error = false;

    let bytes = fs::read(filename).unwrap_or_else(|e| { // Read the bytes of the given file, or return an error.
        is_error = true;
        return format!("{}", e).red().as_bytes().to_vec();
    });

    let mut string = String::from_utf8_lossy(&bytes).to_string(); // Convert the bytes to a string.
    
    if is_error { string = string.red().to_string(); } // If an error occurred, color the string red.

    string
}

fn main() {
    let args = common::parse_args();
    if args.is_empty() { println!("{}", "Please supply a file name.".red()); } // If no file name is supplied, print an error.
    else { println!("{}", read_to_string(&args[0])); }
}