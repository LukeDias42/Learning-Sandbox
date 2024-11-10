
pub struct Maze {
    pub start: Point,
    pub rows: usize,
    pub columns: usize,
    pub cell_width: usize,
    pub cell_height: usize,
    pub cells: Vec<Vec<Cell>>,
}

