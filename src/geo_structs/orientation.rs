/// Enum representing a direction of a line
/// given three ponts $p$ $q$ and $r$. It can be `Collinear` when
/// the three points lie in the same line;
/// `Clockwise` when the third point $r$ lies to the left of the line
/// defined by $p$ and $q$ or `CounterClockwise` if $r$ lies in the right.
pub enum Direction {
    Collinear,
    Clockwise,
    CounterClockwise,
}
