
pub struct Maze {
    pub start: Point,
    pub rows: usize,
    pub columns: usize,
    pub cell_width: usize,
    pub cell_height: usize,
    pub cells: Vec<Vec<Cell>>,
}

impl Maze {
    pub fn new(
        start: Point,
        rows: usize,
        columns: usize,
        cell_width: usize,
        cell_height: usize,
    ) -> Maze {
        let cells = Maze::create_cells(start, rows, columns, cell_width, cell_height);
        let mut maze = Maze {
            start,
            rows,
            columns,
            cell_width,
            cell_height,
            cells,
        };
        maze.break_entrance_and_exit();
        maze
    }
}
