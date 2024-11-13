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
