use super::super::geo_structs::point::Point;
use super::super::geo_structs::orientation::{Direction, orientation};

/// Function to detect  if a given point  is inside a triangle  defined by three
/// points.
///
/// A point is  said to be inside a  triangle iff the direction is  the same for
/// all the three lines of the triangle.
///
/// ## Params:
/// - `p`: first point of the triangle.
/// - `q`: second point of the triangle.
/// - `r`: third point of the triangle.
/// - `curr`: the point to detect if it's inside a triangle defined by `p`, `q`
/// and `r`.
///
/// ## Returns:
/// `true` if the point `curr` is inside a triangle, otherwise returns `false`.
fn is_point_iside_triangle(p: &Point, q: &Point, r: &Point, curr: &Point) -> bool {
    let or1: Direction = orientation(p, q, curr);
    let or2: Direction = orientation(q, r, curr);
    let or3: Direction = orientation(r, p, curr);

    let mut clockw: Vec<bool> = vec![false ; 3];
    let mut countclock: Vec<bool> = vec![false ; 3];

    match or1 {
        Direction::Collinear => {}
        Direction::Clockwise => {clockw[0] = true}
        Direction::CounterClockwise => {countclock[0] = true},
        Direction::None => {}
    }

    match or2 {
        Direction::Collinear => {}
        Direction::Clockwise => {clockw[1] = true}
        Direction::CounterClockwise => {countclock[1] = true},
        Direction::None => {}
    }

    match or3 {
        Direction::Collinear => {}
        Direction::Clockwise => {clockw[2] = true}
        Direction::CounterClockwise => {countclock[2] = true},
        Direction::None => {}
    }

    if clockw.iter().all(|&x| x) || countclock.iter().all(|&x| x) {return true};

    false
}

/// Algorithm of extreme points. It calculates the Convex Hull of a given set of
/// points by  checking, for  every point in  the set, if  it's contained  in at
/// least a tringle  formed by other three  points in the set, in  which case it
/// ignores it and continues  to the next point; otherwise it  adds the point to
/// the resulting vector.
///
/// It has complexity of *O(n^4)* since  we compare every point with other three
/// in order to detect if it's inside a triangle or not.
///
/// ## Params:
/// - `points`: the set of point which we want to calculate the Convex
/// Hull.
/// - `n`: the size of the set of points.
///
/// ## Returns:
/// - A `Point` vector which contains the points of the set that conform
/// the Convex Hull.
pub fn extreme_points(points: &Vec<Point>, n: usize) -> Vec<Point> {
    let mut result: Vec<Point> = vec![];

    for point_idx in 0..n {
        let mut is_inside: bool = false;

        for i in 0..n {
            if i == point_idx {continue;}

            for j in i + 1..n {
                if j == point_idx {continue;}

                for k in j + 1..n {
                    if k == point_idx {continue;}

                    let p: &Point = &points[i];
                    let q: &Point = &points[j];
                    let r: &Point = &points[k];
                    let curr: &Point = &points[point_idx];

                    if is_point_iside_triangle(p, q, r, curr){
                        is_inside = true;
                        break;
                    }
                }
                if is_inside {break;}
            }
            if is_inside {break;}
        }
        if !is_inside {
            result.push(points[point_idx]);
        }
    }

    result
}
