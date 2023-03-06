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


    let a@(mut or1_c, mut or2_c, mut or3_c) = (false, false, false);
    let b@(mut or1_cc, mut or2_cc, mut or3_cc) = (false, false, false);

    match or1 {
        Direction::Collinear => {or1_c = true}
        Direction::Clockwise => {}
        Direction::CounterClockwise => {}
        Direction::Invalid => {}
    }

    match or2 {
        Direction::Collinear => {or2_c = true}
        Direction::Clockwise => {}
        Direction::CounterClockwise => {}
        Direction::Invalid => {}
    }

    match or3 {
        Direction::Collinear => {or2_c = true}
        Direction::Clockwise => {}
        Direction::CounterClockwise => {}
        Direction::Invalid => {}
    }
    // let mut or1_c: bool;
    // let mut or2_c: bool;
    // let mut or3_c: bool;

    // let mut or1_cc: bool;
    // let mut or2_cc: bool;
    // let mut or3_cc: bool;
    // if (or1 == Direction::Clockwise &&
    //     or2 == Direction::Clockwise &&
    //     or3 == Direction::Clockwise){
    //     return true;
    // }

    false
}
pub fn _extreme_points() {

}
