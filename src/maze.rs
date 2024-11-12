use minifb::Window;
use rand::Rng;

use crate::cell::Cell;
use crate::color::Color;
use crate::point::Point;

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
    pub fn draw_cell(
        &self,
        x: usize,
        y: usize,
        buffer: &mut Vec<u32>,
        window_width: usize,
        window_height: usize,
    ) {
        let color = Color::new(255, 255, 255);
        let opening_color = Color::new(0x33, 0x33, 0x33);
        self.cells[y][x].plot_cell_lines(color, opening_color, buffer, window_width, window_height);
    }
    pub fn break_entrance_and_exit(&mut self) {
        self.cells[0][0].sides = [true, false, true, true];
        self.cells[self.rows - 1][self.columns - 1].sides = [true, true, true, false];
    }

    pub fn break_walls_r(&mut self, x: usize, y: usize, window_width: usize, window_height: usize) {
        let mut rng = rand::thread_rng();

        self.cells[y][x].visited = true;
        let mut to_visit: Vec<[isize; 2]> = Vec::with_capacity(4);

        if y != 0 && !self.cells[y - 1][x].visited {
            to_visit.push([0, -1]); // Up
        }
        if y < self.rows - 1 && !self.cells[y + 1][x].visited {
            to_visit.push([0, 1]); // Down
        }
        if x != 0 && !self.cells[y][x - 1].visited {
            to_visit.push([-1, 0]); // Left
        }
        if x < self.columns - 1 && !self.cells[y][x + 1].visited {
            to_visit.push([1, 0]); // Right
        }

        if to_visit.is_empty() {
            return;
        }

        while !to_visit.is_empty() {
            let random_direction = rng.gen_range(0..to_visit.len());
            let direction = to_visit[random_direction];

            let new_x = ((x as isize) + direction[0]) as usize;
            let new_y = ((y as isize) + direction[1]) as usize;

            if self.cells[new_y][new_x].visited {
                to_visit.remove(random_direction);
                continue;
            }
            match direction {
                [1, 0] => {
                    self.cells[y][x].sides[2] = false; // Remove right wall of current cell
                    self.cells[new_y][new_x].sides[0] = false; // Remove left wall of new cell
                }
                [-1, 0] => {
                    // Moving left
                    self.cells[y][x].sides[0] = false; // Remove left wall of current cell
                    self.cells[new_y][new_x].sides[2] = false; // Remove right wall of new cell
                }
                [0, 1] => {
                    // Moving down
                    self.cells[y][x].sides[3] = false; // Remove bottom wall of current cell
                    self.cells[new_y][new_x].sides[1] = false; // Remove top wall of new cell
                }
                [0, -1] => {
                    // Moving up
                    self.cells[y][x].sides[1] = false; // Remove top wall of current cell
                    self.cells[new_y][new_x].sides[3] = false; // Remove bottom wall of new cell
                }
                _ => {}
            }

            self.break_walls_r(new_x, new_y, window_width, window_height);

            // Remove the visited direction from the list
            to_visit.remove(random_direction);
        }
    }

    pub fn reset_visited_cells(&mut self) {
        for y in 0..self.rows {
            for x in 0..self.columns {
                self.cells[y][x].visited = false;
            }
        }
    }

}
