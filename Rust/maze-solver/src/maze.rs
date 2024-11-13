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
    start_cell: Point,
    end_cell: Point,
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
        let [start_cell, end_cell] = Maze::choose_start_and_end_cell(columns, rows);
        let mut maze = Maze {
            start,
            rows,
            columns,
            cell_width,
            cell_height,
            cells,
            start_cell,
            end_cell,
        };
        maze.break_entrance_and_exit();
        maze
    }
    fn choose_start_and_end_cell(width: usize, height: usize) -> [Point; 2] {
        let start_cell = Maze::pick_random_outer_cell(width, height);
        let mut end_cell = Maze::pick_random_outer_cell(width, height);
        while start_cell == end_cell {
            end_cell = Maze::pick_random_outer_cell(width, height);
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
    fn break_entrance_and_exit(&mut self) {
        self.cells[self.start_cell.y][self.start_cell.x].sides =
            self.walls_based_on_outer_direction(self.start_cell.x, self.start_cell.y);
        self.cells[self.end_cell.y][self.end_cell.x].sides =
            self.walls_based_on_outer_direction(self.end_cell.x, self.end_cell.y);
    }

    fn walls_based_on_outer_direction(&self, x: usize, y: usize) -> [bool; 4] {
        if x == 0 {
            return [false, true, true, true];
        }
        if y == 0 {
            return [true, false, true, true];
        }
        if x == self.columns - 1 {
            return [true, true, false, true];
        }
        return [true, true, true, false];
    }

    pub fn break_walls(&mut self, window_width: usize, window_height: usize) {
        self.break_walls_r(
            self.start_cell.x,
            self.start_cell.y,
            window_width,
            window_height,
        )
    }

    fn break_walls_r(&mut self, x: usize, y: usize, window_width: usize, window_height: usize) {
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

    pub fn solve(
        &mut self,
        buffer: &mut Vec<u32>,
        window_width: usize,
        window_height: usize,
        window: &mut Window,
    ) -> bool {
        self.solve_r(
            self.start_cell.x,
            self.start_cell.y,
            buffer,
            window_width,
            window_height,
            window,
        )
    }

    fn solve_r(
        &mut self,
        x: usize,
        y: usize,
        buffer: &mut Vec<u32>,
        window_width: usize,
        window_height: usize,
        window: &mut Window,
    ) -> bool {
        self.cells[y][x].visited = true;
        if x == self.end_cell.x && y == self.end_cell.y {
            return true;
        }

        let mut not_visited: Vec<[isize; 2]> = Vec::with_capacity(4);
        if x != 0 && !self.cells[y][x].sides[0] {
            // left
            if !self.cells[y][x - 1].visited {
                not_visited.push([-1, 0]);
            }
        }
        if y != 0 && !self.cells[y][x].sides[1] {
            // up
            if !self.cells[y - 1][x].visited {
                not_visited.push([0, -1]);
            }
        }
        if x < self.columns - 1 && !self.cells[y][x].sides[2] {
            // right
            if !self.cells[y][x + 1].visited {
                not_visited.push([1, 0]);
            }
        }
        if y < self.rows - 1 && !self.cells[y][x].sides[3] {
            // down
            if !self.cells[y + 1][x].visited {
                not_visited.push([0, 1]);
            }
        }
        for cell in not_visited {
            self.cells[y][x].move_cell(
                &self.cells[((y as isize) + cell[1]) as usize][((x as isize) + cell[0]) as usize],
                false,
                buffer,
                window_width,
                window_height,
            );
            window
                .update_with_buffer(&buffer, window_width, window_height)
                .unwrap();
            if self.solve_r(
                ((x as isize) + cell[0]) as usize,
                ((y as isize) + cell[1]) as usize,
                buffer,
                window_width,
                window_height,
                window,
            ) {
                return true;
            } else {
                self.cells[y][x].move_cell(
                    &self.cells[((y as isize) + cell[1]) as usize]
                        [((x as isize) + cell[0]) as usize],
                    true,
                    buffer,
                    window_width,
                    window_height,
                );
            }
            window
                .update_with_buffer(&buffer, window_width, window_height)
                .unwrap();
        }
        return false;
    }
}
