use crate::color::Color;
use crate::line::Line;
use crate::point::Point;

pub struct CellHex {
    pub sides: [bool; 6],
    pub horizontal_diagonal: Line,
    pub side_len: f64,
    pub visited: bool,
}

const SQRT_3: f64 = 1.732050807568877293527446341505872367_f64;
impl CellHex {
    pub fn new(
        has_down_left_wall: bool,
        has_up_left_wall: bool,
        has_up_wall: bool,
        has_up_right_wall: bool,
        has_down_right_wall: bool,
        has_down_wall: bool,
        horizontal_diagonal: Line,
    ) -> CellHex {
        let mut x1 = horizontal_diagonal.p1.x as isize;
        let mut x2 = horizontal_diagonal.p2.x as isize;
        let mut y1 = horizontal_diagonal.p1.y as isize;
        let mut y2 = horizontal_diagonal.p2.y as isize;
        if horizontal_diagonal.p1.x > horizontal_diagonal.p2.x {
            x1 = horizontal_diagonal.p2.x as isize;
            x2 = horizontal_diagonal.p1.x as isize;
        }
        if horizontal_diagonal.p1.y > horizontal_diagonal.p2.y {
            y1 = horizontal_diagonal.p2.y as isize;
            y2 = horizontal_diagonal.p1.y as isize;
        }
        let normalized_diagonal = Line::new(
            Point::new(x1 as usize, y1 as usize),
            Point::new(x2 as usize, y2 as usize),
        );
        let side_len =
            (((x1 - x2) * (x1 - x2)) as f64 - ((y1 - y2) * (y1 - y2)) as f64).sqrt() / 2_f64;
        CellHex {
            sides: [
                has_down_left_wall,
                has_up_left_wall,
                has_up_wall,
                has_up_right_wall,
                has_down_right_wall,
                has_down_wall,
            ],
            side_len,
            horizontal_diagonal: normalized_diagonal,
            visited: false,
        }
    }
    pub fn plot_cell_lines(
        &self,
        wall_color: Color,
        opening_color: Color,
        buffer: &mut Vec<u32>,
        window_width: usize,
        window_height: usize,
    ) {
        let mut lines = Vec::with_capacity(6);
        let half_height = SQRT_3 * self.side_len / 2_f64;

        // down left - 0
        lines.push(Line::new(
            Point::new(self.horizontal_diagonal.p1.x, self.horizontal_diagonal.p1.y),
            Point::new(
                self.horizontal_diagonal.p1.x + (self.side_len / 2_f64) as usize,
                (self.horizontal_diagonal.p1.y as f64 + half_height) as usize,
            ),
        ));
        // up left - 1
        lines.push(Line::new(
            Point::new(self.horizontal_diagonal.p1.x, self.horizontal_diagonal.p1.y),
            Point::new(
                self.horizontal_diagonal.p1.x + (self.side_len / 2_f64) as usize,
                (self.horizontal_diagonal.p1.y as f64 - half_height) as usize,
            ),
        ));
        // up - 2
        lines.push(Line::new(
            Point::new(
                self.horizontal_diagonal.p1.x + (self.side_len / 2_f64) as usize,
                (self.horizontal_diagonal.p1.y as f64 - half_height) as usize,
            ),
            Point::new(
                self.horizontal_diagonal.p1.x + (3_f64 * self.side_len / 2_f64) as usize,
                (self.horizontal_diagonal.p1.y as f64 - half_height) as usize,
            ),
        ));
        // up right - 3
        lines.push(Line::new(
            Point::new(self.horizontal_diagonal.p2.x, self.horizontal_diagonal.p2.y),
            Point::new(
                self.horizontal_diagonal.p2.x - (self.side_len / 2_f64) as usize,
                (self.horizontal_diagonal.p2.y as f64 - half_height) as usize,
            ),
        ));
        // down right - 4
        lines.push(Line::new(
            Point::new(self.horizontal_diagonal.p2.x, self.horizontal_diagonal.p2.y),
            Point::new(
                self.horizontal_diagonal.p2.x - (self.side_len / 2_f64) as usize,
                (self.horizontal_diagonal.p2.y as f64 + half_height) as usize,
            ),
        ));
        // down - 5
        lines.push(Line::new(
            Point::new(
                self.horizontal_diagonal.p1.x + (self.side_len / 2_f64) as usize,
                (self.horizontal_diagonal.p1.y as f64 + half_height) as usize,
            ),
            Point::new(
                self.horizontal_diagonal.p1.x + (3_f64 * self.side_len / 2_f64) as usize,
                (self.horizontal_diagonal.p1.y as f64 + half_height) as usize,
            ),
        ));
        for wall in 0..self.sides.len() {
            let color = if self.sides[wall] {
                wall_color
            } else {
                opening_color
            };
            lines[wall].plot_bresenham_line(&color, buffer, window_width, window_height);
        }
    }
}
