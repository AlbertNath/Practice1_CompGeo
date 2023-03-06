// Module tree declaration
mod i_o;
mod geo_structs;
mod algorithmia;

// Imports
use std::env;
use std::process;
use crate::i_o::reader::Reader;

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
    let rdr = Reader {};

    //println!("{:?}", rdr.parse_points(in_file));
    println!("{:?}", rdr.parse_points(in_file));
}
