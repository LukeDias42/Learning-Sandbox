use crate::color::Color;
use crate::point::Point;
use std::isize;

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

    pub fn plot_bresenham_line(
        &self,
        color: &Color,
        buffer: &mut Vec<u32>,
        window_width: usize,
        window_height: usize,
    ) {
        let dx = (self.p2.x as isize) - (self.p1.x as isize);
        let dy = (self.p2.y as isize) - (self.p1.y as isize);

        let step_x: isize = if self.p1.x < self.p2.x { 1 } else { -1 };
        let step_y: isize = if self.p1.y < self.p2.y { 1 } else { -1 };

        let mut err = dx.abs() - dy.abs();

        let mut x = self.p1.x as isize;
        let mut y = self.p1.y as isize;
        let goal_x = self.p2.x as isize;
        let goal_y = self.p2.y as isize;

        let width = window_width as isize;
        let height = window_height as isize;

        let color = color.to_rgb888();

        loop {
            if x >= 0 && x < width && y >= 0 && y < height {
                buffer[x as usize + window_width * y as usize] = color;
            }

            if x == goal_x && y == goal_y {
                break;
            }

            let e2 = 2 * err;

            if e2 > -dy.abs() {
                err -= dy.abs();
                x += step_x;
            }

            // Move in the y direction
            if e2 < dx.abs() {
                err += dx.abs();
                y += step_y;
            }
        }
    }

    pub fn middle(&self) -> Point {
        let x = (self.p1.x + self.p2.x) / 2;
        let y = (self.p1.y + self.p2.y) / 2;
        Point::new(x, y)
    }
}
