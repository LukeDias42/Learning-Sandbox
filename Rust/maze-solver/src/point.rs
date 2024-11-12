use crate::color::Color;
use crate::line::Line;

#[derive(Clone, Copy, Debug)]
pub struct Point {
    pub x: usize,
    pub y: usize,
}

impl Point {
    pub fn new(x: usize, y: usize) -> Point {
        Point { x, y }
    }
    pub fn to(&self, other: Point) -> Line {
        Line {
            p1: self.clone(),
            p2: other,
        }
    }
    pub fn plot_point(
        &self,
        color: Color,
        buffer: &mut Vec<u32>,
        window_width: usize,
        window_height: usize,
    ) {
        let color = color.to_rgb888();
        if self.x < window_width && self.y < window_height {
            buffer[self.x + window_width * self.y] = color;
        }
    }
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generates_line_correctly() {
        let p1 = Point { x: 0, y: 0 };
        let p2 = Point { x: 500, y: 300 };
        let line = p1.to(p2);

        assert_eq!(
            line,
            Line {
                p1: Point { x: 0, y: 0 },
                p2: Point { x: 500, y: 300 }
            }
        );
    }
}
