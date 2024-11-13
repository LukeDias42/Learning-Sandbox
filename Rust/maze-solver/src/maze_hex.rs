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

}
