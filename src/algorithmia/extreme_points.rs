use super::super::geo_structs::point::Point;
use super::super::geo_structs::orientation::Direction;

fn orientation(p1: &Point, p2: &Point, p3: &Point) -> Direction {
    let value: i32 = (p3.y - p1.y) * (p2.x - p1.x) - (p2.y - p1.y) * (p3.x - p1.x);

    if value == 0 {
        return Direction::Collinear;
    } else if value > 0  {
        return Direction::CounterClockwise;
    } else if value < 0 {
        return Direction::Clockwise;
    }

    Direction::Invalid
}

fn _is_point_iside_triangle(p: &Point, q: &Point, r: &Point, curr: &Point) -> bool {
    let or1: Direction = orientation(p, q, curr);
    let or2: Direction = orientation(q, r, curr);
    let or3: Direction = orientation(r, p, curr);

    let (mut or1_c, mut or2_c, mut or3_c) = (false, false, false);
    let (mut or1_cc, mut or2_cc, mut or3_cc) = (false, false, false);

    match or1 {
        Direction::Collinear => {}
        Direction::Clockwise => {or1_c = true}
        Direction::CounterClockwise => {or1_cc = true}
        Direction::Invalid => {}
    }

    match or2 {
        Direction::Collinear => {}
        Direction::Clockwise => {or2_c = true}
        Direction::CounterClockwise => {or2_cc = true}
        Direction::Invalid => {}
    }

    match or3 {
        Direction::Collinear => {}
        Direction::Clockwise => {or3_c = true}
        Direction::CounterClockwise => {or3_cc = true}
        Direction::Invalid => {}
    }

    if or1_c && or2_c && or3_c { return true };

    if or1_cc && or2_cc && or3_cc { return true };
    false
}

pub fn _extreme_points(points: Vec<Point>, n: usize) {
    let mut convex_hull: Vec<Point> = vec![];
}
