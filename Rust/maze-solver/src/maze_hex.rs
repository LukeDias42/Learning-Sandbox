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
    pub fn animate(
        start: Point,
        rows: usize,
        columns: usize,
        cell_side_len: usize,
        background_color: Color,
        window_width: usize,
        window_height: usize,
        mut buffer: &mut Vec<u32>,
        mut window: &mut Window,
    ) {
        buffer.fill(background_color.to_rgb888());
        let mut maze = MazeHex::new(start, rows, columns, cell_side_len);
        maze.break_walls(window_width, window_height);
        maze.reset_visited_cells();
        maze.draw_maze(&mut buffer, window_width, window_height);
        maze.solve(&mut buffer, window_width, window_height, &mut window);
    }

    pub fn new(start: Point, rows: usize, columns: usize, cell_side_len: usize) -> MazeHex {
        let cells = MazeHex::create_cells(start, rows, columns, cell_side_len);
        let [start_cell, end_cell] = MazeHex::choose_start_and_end_cell(columns, rows);
        let mut maze = MazeHex {
            start,
            rows,
            columns,
            cell_side_len,
            cells,
            start_cell,
            end_cell,
        };
        maze.break_entrance_and_exit();
        maze
    }

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

    pub fn draw_maze(&self, mut buffer: &mut Vec<u32>, window_width: usize, window_height: usize) {
        for y in 0..self.rows {
            for x in 0..self.columns {
                self.draw_cell(x, y, &mut buffer, window_width, window_height);
            }
        }
    }

    fn break_entrance_and_exit(&mut self) {
        self.cells[self.start_cell.y][self.start_cell.x].sides =
            self.walls_based_on_outer_direction(self.start_cell.x, self.start_cell.y);
        self.cells[self.end_cell.y][self.end_cell.x].sides =
            self.walls_based_on_outer_direction(self.end_cell.x, self.end_cell.y);
    }

    fn walls_based_on_outer_direction(&self, x: usize, y: usize) -> [bool; 6] {
        if x == 0 {
            return [false, true, true, true, true, true];
        }
        if y == 0 {
            return [true, true, false, true, true, true];
        }
        if x == self.columns - 1 {
            return [true, true, true, false, true, true];
        }
        return [true, true, true, true, true, false];
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
        let mut to_visit: Vec<[isize; 2]> = Vec::with_capacity(6);

        if y != 0 && !self.cells[y - 1][x].visited {
            to_visit.push([0, -1]); // Up
        }
        if y < self.rows - 1 && !self.cells[y + 1][x].visited {
            to_visit.push([0, 1]); // Down
        }
        if x % 2 == 0 {
            if x != 0 && y != 0 && !self.cells[y - 1][x - 1].visited {
                to_visit.push([-1, -1]); // Up Left
            }
            if x != 0 && !self.cells[y][x - 1].visited {
                to_visit.push([-1, 0]); // Down Left
            }
            if x < self.columns - 1 && y != 0 && !self.cells[y - 1][x + 1].visited {
                to_visit.push([1, -1]); // Up Right
            }
            if x < self.columns - 1 && !self.cells[y][x + 1].visited {
                to_visit.push([1, 0]); // Down Right
            }
        } else {
            if x != 0 && !self.cells[y][x - 1].visited {
                to_visit.push([-1, 0]); // Up Left
            }
            if x != 0 && y < self.rows - 1 && !self.cells[y + 1][x - 1].visited {
                to_visit.push([-1, 1]); // Down Left
            }
            if x < self.columns - 1 && y != 0 && !self.cells[y][x + 1].visited {
                to_visit.push([1, 0]); // Up Right
            }
            if x < self.columns - 1 && y < self.rows - 1 && !self.cells[y + 1][x + 1].visited {
                to_visit.push([1, 1]); // Down Right
            }
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
                [-1, -1] => {
                    // Moving Up Left
                    self.cells[y][x].sides[1] = false; // up left
                    self.cells[new_y][new_x].sides[4] = false; // down right
                }
                [1, -1] => {
                    // Moving Up Right
                    self.cells[y][x].sides[3] = false; // up right
                    self.cells[new_y][new_x].sides[0] = false; // down left
                }
                [1, 0] => {
                    if x % 2 == 0 {
                        self.cells[y][x].sides[4] = false; // down right
                        self.cells[new_y][new_x].sides[1] = false; // up left
                    } else {
                        self.cells[y][x].sides[3] = false; // up right
                        self.cells[new_y][new_x].sides[0] = false; // down left
                    }
                }
                [1, 1] => {
                    // Moving down right
                    self.cells[y][x].sides[4] = false; // down right
                    self.cells[new_y][new_x].sides[1] = false; // up left
                }
                [-1, 1] => {
                    // Moving down left
                    self.cells[y][x].sides[0] = false; // down left
                    self.cells[new_y][new_x].sides[3] = false; // up right
                }
                [-1, 0] => {
                    if x % 2 == 0 {
                        self.cells[y][x].sides[0] = false; // down left
                        self.cells[new_y][new_x].sides[3] = false; // up right
                    } else {
                        self.cells[y][x].sides[1] = false; // up left
                        self.cells[new_y][new_x].sides[4] = false; // down right
                    }
                }
                [0, 1] => {
                    // Moving down
                    self.cells[y][x].sides[5] = false; // down
                    self.cells[new_y][new_x].sides[2] = false; // up
                }
                [0, -1] => {
                    // Moving up
                    self.cells[y][x].sides[2] = false; // up
                    self.cells[new_y][new_x].sides[5] = false; // down
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

        let mut not_visited: Vec<[isize; 2]> = Vec::with_capacity(6);
        if y != 0 && !self.cells[y][x].sides[2] && !self.cells[y - 1][x].visited {
            // up
            not_visited.push([0, -1]);
        }
        if y < self.rows - 1 && !self.cells[y][x].sides[5] && !self.cells[y + 1][x].visited {
            // down
            not_visited.push([0, 1]);
        }
        if x % 2 == 0 {
            if x != 0 && y != 0 && !self.cells[y][x].sides[1] && !self.cells[y - 1][x - 1].visited {
                not_visited.push([-1, -1]); // Up Left
            }
            if x != 0 && !self.cells[y][x].sides[0] && !self.cells[y][x - 1].visited {
                not_visited.push([-1, 0]); // Down Left
            }
            if x < self.columns - 1
                && y != 0
                && !self.cells[y][x].sides[3]
                && !self.cells[y - 1][x + 1].visited
            {
                not_visited.push([1, -1]); // Up Right
            }
            if x < self.columns - 1 && !self.cells[y][x].sides[4] && !self.cells[y][x + 1].visited {
                not_visited.push([1, 0]); // Down Right
            }
        } else {
            if x != 0 && !self.cells[y][x].sides[1] && !self.cells[y][x - 1].visited {
                not_visited.push([-1, 0]); // Up Left
            }
            if x != 0
                && y < self.rows - 1
                && !self.cells[y][x].sides[0]
                && !self.cells[y + 1][x - 1].visited
            {
                not_visited.push([-1, 1]); // Down Left
            }
            if x < self.columns - 1
                && y != 0
                && !self.cells[y][x].sides[3]
                && !self.cells[y][x + 1].visited
            {
                not_visited.push([1, 0]); // Up Right
            }
            if x < self.columns - 1
                && y < self.rows - 1
                && !self.cells[y][x].sides[4]
                && !self.cells[y + 1][x + 1].visited
            {
                not_visited.push([1, 1]); // Down Right
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
