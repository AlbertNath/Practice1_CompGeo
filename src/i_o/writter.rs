use std::{fs, process};
use crate::geo_structs::point::Point;

/// Auxiliary function to write a file and handle
/// errors.
///
/// ## Params:
/// - `data`: the data to write in a file.
fn write_file(data: String) {
    match fs::write("results.txt", data) {
        Ok(()) => (),
        Err(_) => {
            println!("Unable to write resulting file");
            process::exit(1);
        }
    }
}

/// Function to write a result to a file.
///
/// ## Params:
/// - `data`: the data to write in a file.
pub fn write_result(data: &Vec<Point>) {
    let mut to_write: String = String::new();
    for i in data {
        to_write.push_str(format!("{:?}\n", i.id).as_str());
    }

    write_file(to_write)
}
