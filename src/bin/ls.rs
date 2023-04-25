use std::fs::{self, Metadata};

/// The three types of entries ls will differentiate between.
enum EntryType {
    Dir,
    File,
    Error
}

/// Returns a Vec<(String, EntryType)> of the item names in the current working directory accompanied by their type.
fn get_dir_entries_with_type(path: Option<&str>) -> Vec<(String, EntryType)> {
    // If no path is provided, use the current working directory.
    let path_str = path.unwrap_or(".");

    let mut list = Vec::new();

    // Iterate over the directory entries and return a Vec<String> of the items in the directory accompanied by their type.
    // This hurts my soul, imma do my best to explain it. I LOVE Result<T>, I LOOOOVE Result<T>!!!!!!
    match fs::read_dir(path_str) {
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

fn main() {
    // Parse flags, if any.

    // If no flags:

    // If flags:
    //     Act accordingly.
}