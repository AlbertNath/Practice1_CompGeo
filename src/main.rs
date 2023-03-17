// Module tree declaration
mod i_o;
mod geo_structs;
mod algorithmia;

// Imports
use std::env;
use std::process;
use crate::i_o::{reader, writter};
use crate::geo_structs::point::Point;

/// Auxiliary function to show an error message
/// and exit properly.
///
/// ### Params:
/// - `str`: the error message to show.
fn err(msg: &str) {
   println!("{}", msg);
   process::exit(1);
}

/// # Geometría computacional, semestre 2023-2
/// ## Practice 1: Extreme Points.
/// ### Author: [Medel Piña Alberto Natanael](https://github.com/AlbertNath)
///
/// Implementation of [Extreme Points](https://en.wikipedia.org/wiki/Extreme_point)
/// algorithm.
fn main() {

    let args: Vec<String> = env::args().collect();
    if args.len() - 1 == 0{
        err("Too few arguments.");
    }

    let in_file = &args[1];

    let input: Vec<Point> = reader::parse_points(&in_file);
    let result: Vec<Point> = algorithmia::extreme_points::extreme_points(&input,input.len());
    writter::write_result(&result)
}
