use super::super::geo_structs::point::Point;
use super::super::geo_structs::orientation::Direction;

fn _orientation(p1: &Point, p2: &Point, p3: &Point) -> Direction {
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

pub fn _extreme_points() {

}
