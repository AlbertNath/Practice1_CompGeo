use std::{fs, process};

use super::super::geo_structs::point::Point;

/// Auxiliary function to read a file and handle
/// errors.
///
/// ## Params:
/// - `path`: the path where the file will be
/// readed.
///
/// ## Returns:
/// A `String` containing the encoded content.
fn read_file(path: &str) -> String {
    match fs::read_to_string(path) {
        Ok(res) => res,
        Err(_) => {
            println!("Unable to read from file {}", path);
            process::exit(1)
        }
    }
}

/// Function to handle a raw point from the input file.
/// It parses from `&str` type to `i32` in order to do
/// arithmetic operations over the points.
///
/// If it fails to parse, it will end the program and
/// give a message error.
///
/// ## Params:
/// - `p`: the coordinate point to be parsed.
///
/// ## Returns:
/// An `i32` integer.
fn handle_raw_point(p: &str) -> i32 {
    match p.trim().parse::<i32>() {
        Ok(n) => n,
        Err(_) => {
            println!("Bad parsing: couldn't parse string {} to i32", p);
            process::exit(1)
        }
    }
}

/// Function to read and parse the points from the input file.
///
/// ## Params:
/// - `path`: the path where the file will be readed.
///
/// ## Returns:
/// A `Point` vector containing all the parsed points.
pub fn parse_points(path: &str) -> Vec<Point> {
    let raw = read_file(path);
    let mut points: Vec<Point> = Vec::new();

    for p in raw.split(',') {
        let comp: Vec<&str> = p.split(':').collect();
        let new_p = Point {
            id:  handle_raw_point(comp[0]),
            x:  handle_raw_point(comp[1]),
            y:  handle_raw_point(comp[2]),
        };

        points.push(new_p);
    }

    points
}
