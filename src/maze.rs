
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
    fn create_cells(
        start: Point,
        rows: usize,
        columns: usize,
        cell_width: usize,
        cell_height: usize,
    ) -> Vec<Vec<Cell>> {
        let mut cells: Vec<Vec<Cell>> = Vec::with_capacity(rows);
        for y in 0..rows {
            let mut row: Vec<Cell> = Vec::with_capacity(columns);
            for x in 0..columns {
                let diagonal =
                    Point::new(x * cell_width + start.x, y * cell_height + start.y).to(Point::new(
                        start.x + (x + 1) * cell_width,
                        start.y + (y + 1) * cell_height,
                    ));
                let cell = Cell::new(true, true, true, true, diagonal);
                row.push(cell);
            }
            cells.push(row);
        }
        cells
    }
}
