use minifb::Window;
use rand::Rng;

use crate::cell_hex::CellHex;
use crate::color::Color;
use crate::point::Point;

pub struct MazeHex {
    pub start: Point,
    pub rows: usize,
    pub columns: usize,
    pub cell_side_len: usize,
    pub cells: Vec<Vec<CellHex>>,
    start_cell: Point,
    end_cell: Point,
}

const SQRT_3: f64 = 1.732050807568877293527446341505872367_f64;
impl MazeHex {
    fn choose_start_and_end_cell(width: usize, height: usize) -> [Point; 2] {
        let start_cell = MazeHex::pick_random_outer_cell(width, height);
        let mut end_cell = MazeHex::pick_random_outer_cell(width, height);
        while start_cell == end_cell {
            end_cell = MazeHex::pick_random_outer_cell(width, height);
        }
        return [start_cell, end_cell];
    }
    fn pick_random_outer_cell(width: usize, height: usize) -> Point {
        let mut rng = rand::thread_rng();
        let random_direction = rng.gen_range(0..=3);
        if random_direction == 0 {
            return Point::new(0, rng.gen_range(0..height));
        }
        if random_direction == 1 {
            return Point::new(rng.gen_range(0..width), 0);
        }
        if random_direction == 2 {
            return Point::new(width - 1, rng.gen_range(0..height));
        }
        Point::new(rng.gen_range(0..width), height - 1)
    }

    fn create_cells(
        start: Point,
        rows: usize,
        columns: usize,
        cell_side_len: usize,
    ) -> Vec<Vec<CellHex>> {
        let height = SQRT_3 * cell_side_len as f64;
        let half_height = height / 2_f64;
        let mut cells: Vec<Vec<CellHex>> = Vec::with_capacity(rows);
        for y in 0..rows {
            let mut row: Vec<CellHex> = Vec::with_capacity(columns);

            let mut x_stagger = 0;
            for x in 0..columns {
                let y_stagger = (x % 2) as f64 * half_height;
                let diagonal = Point::new(
                    x * cell_side_len * 2 - x_stagger + start.x,
                    y * (height as usize) + y_stagger as usize + start.y,
                )
                .to(Point::new(
                    (x + 1) * cell_side_len * 2 - x_stagger + start.x,
                    y * (height as usize) + y_stagger as usize + start.y,
                ));
                let cell = CellHex::new(true, true, true, true, true, true, diagonal);
                row.push(cell);
                x_stagger += cell_side_len / 2;
            }
            cells.push(row);
        }
        cells
    }
    pub fn draw_cell(
        &self,
        x: usize,
        y: usize,
        buffer: &mut Vec<u32>,
        window_width: usize,
        window_height: usize,
    ) {
        let color = Color::new(0x8A, 0x9B, 0xA8);
        let opening_color = Color::new(0x2B, 0x2D, 0x42);
        self.cells[y][x].plot_cell_lines(color, opening_color, buffer, window_width, window_height);
    }
}
