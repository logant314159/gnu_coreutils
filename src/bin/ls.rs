use std::fs;
use gnu_coreutils::common;

extern crate colored;

use colored::*;

/// The three types of entries ls will differentiate between.
enum EntryType {
    Dir,
    File,
    Error
}

/// Returns a Vec<(String, EntryType)> of the item names in the current working directory accompanied by their type.
fn get_dir_entries_with_type(path: Option<String>) -> Vec<(String, EntryType)> {
    // If no path is provided, use the current working directory.
    let path_str = path.unwrap_or(".".to_string());

    let mut list = Vec::new();

    // Iterate over the directory entries and return a Vec<String> of the items in the directory accompanied by their type.
    // This hurts my soul, imma do my best to explain it. I LOVE Result<T>, I LOOOOVE Result<T>!!!!!!
    match fs::read_dir(path_str) { // TODO: Refactor this for the love of god.
        Err(e) => list.push((format!("{:?}", e), EntryType::Error)), // If it errors out in the beginning, return an error.
        Ok(entries_iterator) => {
            for entry in entries_iterator {
                match entry {
                    Err(e) => list.push((format!("{:?}", e), EntryType::Error)), // If any entry individually returns an error, append an error to the vector.
                    Ok(entry) => {
                        match entry.metadata() {
                            Err(e) => list.push((format!("{:?}", e), EntryType::Error)), // If any entry's metadata returns an error, append an error to the vector. (God I love Result<T>!)
                            Ok(metadata) => {
                                if metadata.is_dir() {
                                    list.push((entry.file_name().into_string().unwrap(), EntryType::Dir)); // If the entry is a directory, append a directory entry to the vector.
                                } else if metadata.is_file() {
                                    list.push((entry.file_name().into_string().unwrap(), EntryType::File)); // If the entry is a file, append a file entry to the vector.
                                } else {
                                    list.push((entry.file_name().into_string().unwrap(), EntryType::Error)); // If the entry is neither a directory nor a file, append an error entry to the vector.
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    list
}

/// Prints Vec<(String, EntryType)> items colored by type to the console.
fn print_entries(entries: &Vec<(String, EntryType)>, args: &[String]) {
    for entry in entries {
        if entry.0.starts_with('.') && !args.contains(&String::from("-a")) { continue; } // Skip any entries that start with a period.
        match entry.1 {
            EntryType::Dir => println!("{}/", entry.0.blue()),
            EntryType::Error => println!("{}@", entry.0.red()),
            EntryType::File => println!("{}", entry.0)
        }
    }
}

fn main() {
    let mut path = None;
    let args = common::parse_args();

    let mut counter = 0;
    while counter < args.len() {
        if counter == 0 && !args[counter].starts_with('-') { path = Some(args[0].clone().to_string()); }

        counter += 1;
    }

    let entry_list = get_dir_entries_with_type(path);

    print_entries(&entry_list, &args);
}