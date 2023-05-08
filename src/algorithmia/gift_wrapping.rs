use super::super::geo_structs::point::{Point, compare_points};
use super::super::geo_structs::orientation::{Direction, orientation};

/// Jarvis-March Algorithm to compute the Convex Hull of a given
/// set of points. Also known as "Gift Wrapping".
///
/// The algorithm works as follows: we first find an extreme point
/// (either respect to y-coordinate or x-coordinate), then
/// we find the closest point to the former in order to create a
/// proper Convex Hull. We iterate this process until we find
/// again the starting point.
///
/// Time complexity of this algortihm is O(n^2), since we need to
/// compare every point with n-1 points in the worst case scenario.
///
/// ## Params:
/// - `points`: the set of points which we want to calculate its Convex Hull.
/// - `n`: the size of the set of points.
///
/// ## Returns:
/// - A `Point` vector which contains the points of the set that conform
/// the Convex Hull.
pub fn jarvis_march(points: &Vec<Point>, n: usize) -> Vec<Point> {
    let mut convex_hull: Vec<Point> = vec![];

    let mut min_point_idx: usize = 0;
    for i in 0..n {
        let p: Point = points[min_point_idx];
        let q: Point = points[i];
        if !compare_points(&p, &q) {
            min_point_idx = i;
        }
    }

    let mut current_idx: usize = min_point_idx;
    let p: Point = points[current_idx];
    convex_hull.push(p);

    loop {
        let mut next_point_idx: usize = (current_idx + 1) % n;
        for i in 0..n {
            if i == current_idx || i == next_point_idx {
                continue;
            }
            let p: Point = points[current_idx];
            let q: Point = points[i];
            let r: Point = points[next_point_idx];

            let current_orientation: Direction = orientation(&p, &q, &r);
            if current_orientation == Direction::CounterClockwise {
                next_point_idx = i;
            }
        }
        current_idx = next_point_idx;
        if current_idx == min_point_idx {
            break;
        }
        convex_hull.push(points[current_idx]);
    }
    convex_hull
}
