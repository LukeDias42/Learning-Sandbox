use crate::line::Line;
use crate::point::Point;
pub struct Cell {
    pub sides: [bool; 4],
    pub diagonal: Line,
    pub visited: bool,
}

impl Cell {
    pub fn new(
        has_right_wall: bool,
        has_up_wall: bool,
        has_left_wall: bool,
        has_down_wall: bool,
        diagonal: Line,
    ) -> Cell {
        let mut x1 = diagonal.p1.x;
        let mut x2 = diagonal.p2.x;
        let mut y1 = diagonal.p1.y;
        let mut y2 = diagonal.p2.y;
        if diagonal.p1.x > diagonal.p2.x {
            x1 = diagonal.p2.x;
            x2 = diagonal.p1.x;
        }
        if diagonal.p1.y > diagonal.p2.y {
            y1 = diagonal.p2.y;
            y2 = diagonal.p1.y;
        }
        let normalized_diagonal = Line::new(Point::new(x1, y1), Point::new(x2, y2));
        Cell {
            sides: [has_right_wall, has_up_wall, has_left_wall, has_down_wall],
            diagonal: normalized_diagonal,
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
        let mut lines = Vec::with_capacity(4);
        lines.push(Line::new(
            Point::new(self.diagonal.p1.x, self.diagonal.p1.y),
            Point::new(self.diagonal.p1.x, self.diagonal.p2.y),
        ));
        lines.push(Line::new(
            Point::new(self.diagonal.p1.x, self.diagonal.p1.y),
            Point::new(self.diagonal.p2.x, self.diagonal.p1.y),
        ));
        lines.push(Line::new(
            Point::new(self.diagonal.p2.x, self.diagonal.p1.y),
            Point::new(self.diagonal.p2.x, self.diagonal.p2.y),
        ));
        lines.push(Line::new(
            Point::new(self.diagonal.p1.x, self.diagonal.p2.y),
            Point::new(self.diagonal.p2.x, self.diagonal.p2.y),
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
