/// Struct representing a `Point` in the plane with
/// it's $x$  and $y$ coordinates.
#[derive(Debug)]
#[derive(Clone, Copy)]
pub struct Point {
    pub id: i32,
    pub x: i32,
    pub y: i32,
}

/// Function to compare two poins first by y-coordinate. If two
/// points share the same y-coordinate, then we compare
/// lexicographlically with the x-coordinate.
///
/// ## Params:
/// - `p`: first point to compare.
/// - `q`: second point to compare.
///
/// ## Returns:
/// - `true` if `p` is less tan `q` lexicographically or
/// `false` otherwise.
pub fn compare_points (p: &Point, q: &Point) -> bool {
    if p.y < q.y {
        return true;
    }
    if p.y == q.y && p.x < q.x {
        return true;
    }
    false
}
