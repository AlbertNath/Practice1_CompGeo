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

    let mut clockw: Vec<bool> = vec![false ; 3];
    let mut countclock: Vec<bool> = vec![false ; 3];

    match or1 {
        Direction::Collinear => {}
        Direction::Clockwise => {clockw[0] = true}
        Direction::CounterClockwise => {countclock[0] = true}
        Direction::Invalid => {}
    }

    match or2 {
        Direction::Collinear => {}
        Direction::Clockwise => {clockw[1] = true}
        Direction::CounterClockwise => {countclock[1] = true}
        Direction::Invalid => {}
    }

    match or3 {
        Direction::Collinear => {}
        Direction::Clockwise => {clockw[2] = true}
        Direction::CounterClockwise => {countclock[2] = true}
        Direction::Invalid => {}
    }

    if clockw.iter().all(|&x| x) {return true};
    if countclock.iter().all(|&x| x) {return true};

    false
}

pub fn _extreme_points(points: Vec<Point>, n: usize) {
    let mut convex_hull: Vec<Point> = vec![];
}
