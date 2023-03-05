use std::{fs, process};

use super::super::geo_structs::point::Point;

pub struct Reader {}

impl Reader {
    fn read_file(&self, path: &str) -> String {
        match fs::read_to_string(path) {
            Ok(res) => res,
            Err(_) => {
                println!("Unable to read from file {}", path);
                process::exit(1)
            }
        }
    }

    fn handle_raw_point(&self, p: &str) -> i32 {
        match p.trim().parse::<i32>() {
            Ok(n) => n,
            Err(_) => {
                println!("Bad parsing: couldn't parse string {} to i32", p);
                process::exit(1)
            }
        }
    }

    pub fn parse_points(&self, path: &str) -> Vec<Point> {
        let raw = self.read_file(path);
        let mut points: Vec<Point> = Vec::new();

        for p in raw.split(',') {
            let comp: Vec<&str> = p.split(':').collect();
            let new_p = Point {
                id: self.handle_raw_point(comp[0]),
                x: self.handle_raw_point(comp[1]),
                y: self.handle_raw_point(comp[2]),
            };

            points.push(new_p);
        }

        points
    }
}
