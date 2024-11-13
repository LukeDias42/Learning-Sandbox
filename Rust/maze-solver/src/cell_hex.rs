
pub struct CellHex {
    pub sides: [bool; 6],
    pub horizontal_diagonal: Line,
    pub side_len: f64,
    pub visited: bool,
}

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
}
