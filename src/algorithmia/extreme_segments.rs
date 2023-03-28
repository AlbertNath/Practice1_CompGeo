use super::super::geo_structs::point::Point;
use super::super::geo_structs::orientation::{Direction, orientation};

pub fn extreme_segments(points: &Vec<Point>, n: usize) -> Vec<Point> {
    let mut ch: Vec<Point> = vec![];

    for i in 1..n {
        for j in i + 1..n {
            let mut is_extreme_segment: bool = true;
            let mut initial_orientation: Direction = Direction::None;

            for k in 1..n{
                if k == i || k == j {
                    continue;
                }
                let p: Point = points[i];
                let q: Point = points[j];
                let r: Point = points[k];

                if let Direction::None = initial_orientation {
                    initial_orientation = orientation(&p, &q, &r);
                } else {
                    let new_orientation: Direction = orientation(&p, &q, &r);
                    if initial_orientation != new_orientation {
                        is_extreme_segment = false;
                        break;
                    }
                }
            }
            if is_extreme_segment {
                let p: Point = points[i];
                let q: Point = points[j];
                ch.push(p);
                ch.push(q);
            }
        }
    }

    ch
}
