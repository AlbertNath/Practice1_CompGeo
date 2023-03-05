use std::fs;

use super::super::geo_structs::point::Point;

pub struct Reader {}

impl Reader {
    fn read_file(&self, path: &str) -> String {
        fs::read_to_string(path).expect("Unable to read file.")
    }

    pub fn parse_points(&self, path: &str)  {
        let raw = self.read_file(path);
        let content = raw.split(',');
        let mut points: Vec<Point> = Vec::new();

        for p in content {
            let comp: Vec<&str> = p.split(":").collect();
            let new_p = Point {
                id: comp[0].parse().unwrap(),
                x: comp[1].parse().unwrap(),
                y: comp[2].parse().unwrap()
            };

            points.push(new_p);
        }

        println!("{:?}", points);
    }
}
