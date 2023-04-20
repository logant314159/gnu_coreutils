use std::fs;

/// Returns a Vec<String> of the items in the current working directory.
fn raw_list() -> Vec<String> {
    let mut list = Vec::new();

    match fs::read_dir(".") {
        Err(e) => list.push(format!("{e}")),
        Ok(entries) => {
            for entry in entries {
                let result = entry.unwrap().file_name().into_string().unwrap();
                list.push(result);
            }
        }
    }

    list
}

fn main() {
    // Parse flags, if any.

    // If no flags:
    for entry in raw_list() {
        println!("{entry}");
    }

    // If flags:
    //     Act accordingly.
}