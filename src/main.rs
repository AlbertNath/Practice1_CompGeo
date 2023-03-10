// Module tree declaration
mod i_o;
mod geo_structs;
mod algorithmia;

// Imports
use std::env;
use std::process;
use crate::i_o::{reader, writter};
use crate::geo_structs::point::Point;

fn err(msg: &str) {
   println!("{}", msg);
   process::exit(1);
}

fn main() {

    let args: Vec<String> = env::args().collect();
    if args.len() - 1 == 0{
        err("Too few arguments.");
    }

    let in_file = &args[1];

    let input: Vec<Point> = reader::parse_points(&in_file);
    let result: Vec<Point> = algorithmia::extreme_points::_extreme_points(&input,input.len());
    writter::write_result(&result)
}
