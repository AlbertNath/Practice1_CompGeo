use super::super::geo_structs::point::Point;

/// Enum representing a direction of a line
/// given three ponts $p$ $q$ and $r$. It can be `Collinear` when
/// the three points lie in the same line;
/// `Clockwise` when the third point $r$ lies to the left of the line
/// defined by $p$ and $q$ or `CounterClockwise` if $r$ lies in the right.
#[derive(PartialEq)]
pub enum Direction {
    Collinear,
    Clockwise,
    CounterClockwise,
    None
}

/// Function to get the orientation of a point given other two points.
///
/// ## Params:
/// - `p1`: first point of the line.
/// - `p2`: the second point of the line.
/// - `p3`: the point we want to know it's direction.
///
/// ## Returns:
/// - `Direction`: an enum of type `Direction`.
pub fn orientation(p1: &Point, p2: &Point, p3: &Point) -> Direction {
    let value: i32 = (p3.y - p1.y) * (p2.x - p1.x) - (p2.y - p1.y) * (p3.x - p1.x);

    if value == 0 {
        return Direction::Collinear;
    } else if value > 0  {
        return Direction::CounterClockwise;
    } else {
        return Direction::Clockwise;
    }
}
