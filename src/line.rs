use crate::color::Color;
use crate::point::Point;

#[derive(Clone, Copy, Debug)]
pub struct Line {
    pub p1: Point,
    pub p2: Point,
}

impl PartialEq for Line {
    fn eq(&self, other: &Self) -> bool {
        self.p1 == other.p1 && self.p2 == other.p2
    }
}

impl Line {
    pub fn new(p1: Point, p2: Point) -> Line {
        Line { p1, p2 }
    }

}
