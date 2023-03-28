use std::{fs, process};

use super::super::geo_structs::point::Point;
use super::super::algorithmia::algorithms::Algorithm;

/// Auxiliary function to show an error message
/// and exit properly.
///
/// ### Params:
/// - `str`: the error message to show.
pub fn err(msg: &str) {
   println!("{}", msg);
   process::exit(1);
}

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

fn get_flag(args: &Vec<String>) -> Option<&str> {
    for a in args.iter(){
        if a.starts_with("-") {
            return Some(a)
        }
    }
    None
}

fn get_file(args: &Vec<String>) -> Option<&str> {
    for i in 1..args.len() {
        if !args[i].starts_with("-") {
            return  Some(&args[i]);
        }
    }
    None
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
pub fn parse_points(args: &Vec<String>) -> Vec<Point> {
    let mut file: &str = "";

    if let Some(f) = get_file(args) {
        file = f;
    } else {
        err("File not provided.")
    }

    let raw = read_file(file);
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

pub fn get_algorithm(args: &Vec<String>) -> Option<Algorithm> {
    let mut f: &str = "";

    if let Some(i) = get_flag(args) {
        f = i;
    } else {
        err("Invalid or missing flag.");
    }

    match f {
        "-p" => Some(Algorithm::ExtremePoints),
        "-s" => Some(Algorithm::ExtremeSegments),
        _ => None
    }
}
