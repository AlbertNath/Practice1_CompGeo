use super::super::geo_structs::point::Point;
use super::super::geo_structs::orientation::{Direction, orientation};

/// Algorithm of extreme segments. It computes the Convex Hull of a given set of
/// points by checking, for every pair of points in the set, if it's line contanis
/// in _only one side_ all the remainig points of the set; this is called an _extreme
/// segment_.
///
/// The complexity for this algorithm is a little lower than **Extreme points**; since
/// we compare every possible pair of points with the others in the set, we got O(n^3).
///
/// ## Params:
/// - `points`: the set of point which we want to calculate the Convex Hull.
/// - `n`: the size of the set of points.
/// ## Returns:
/// - A `Point` vector which contains the points of the set that conform
/// the Convex Hull.
pub fn extreme_segments(points: &Vec<Point>, n: usize) -> Vec<Point> {
    let mut ch: Vec<Point> = vec![];

    for i in 0..n {
        for j in i + 1..n {
            let mut is_extreme_segment: bool = true;
            let mut initial_orientation: Direction = Direction::None;

            for k in 0..n{
                if k == i || k == j {
                    continue;
                }
                let p: Point = points[i];
                let q: Point = points[j];
                let r: Point = points[k];

                if  initial_orientation == Direction::None {
                    initial_orientation = orientation(&p, &q, &r);
                } else {
                    let new_orientation: Direction = orientation(&p, &q, &r);
                    if new_orientation != initial_orientation {
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
