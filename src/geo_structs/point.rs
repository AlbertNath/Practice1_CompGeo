/// Struct representing a `Point` in the plane with
/// it's $x$  and $y$ coordinates.
#[derive(Debug)]
#[derive(Clone, Copy)]
pub struct Point {
    pub id: i32,
    pub x: i32,
    pub y: i32,
}

pub fn compare_points (p: &Point, q: &Point) -> bool {
    if p.y < q.y {
        return true;
    }
    if p.y == q.y && p.x < q.x {
        return true;
    }
    false
}
