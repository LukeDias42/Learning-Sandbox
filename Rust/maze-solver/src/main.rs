mod cell;
mod cell_hex;
mod color;
mod line;
mod maze;
mod maze_hex;
mod point;

use core::panic;
use maze::Maze;
use maze_hex::MazeHex;
use minifb::{Key, KeyRepeat, Window, WindowOptions};
use point::Point;

const WIDTH: usize = 1280; // 16
const HEIGHT: usize = 720; // 9
fn main() {
    let mut window = Window::new("Maze Solver", WIDTH, HEIGHT, WindowOptions::default())
        .unwrap_or_else(|e| panic!("Failed to create window: {}", e));
    window.set_target_fps(120);
    let mut buffer: Vec<u32> = vec![0x2B2D42; WIDTH * HEIGHT];
    let mut is_hex = true;
    let mut is_paused = true;
    while window.is_open() && !window.is_key_down(Key::Escape) {
        if is_paused && window.is_key_pressed(Key::Left, KeyRepeat::No) {
            is_hex = true;
            is_paused = false;
        }
        if is_paused && window.is_key_pressed(Key::Right, KeyRepeat::No) {
            is_hex = false;
            is_paused = false;
        }
        if !is_paused {
            if is_hex {
                let hex_cell_len = 20;
                let start = Point::new(hex_cell_len, hex_cell_len);
                buffer.fill(0x2B2D42);
                let mut maze = MazeHex::new(start, 20, 41, hex_cell_len);
                maze.break_walls(WIDTH, HEIGHT);
                maze.reset_visited_cells();
                for y in 0..maze.rows {
                    for x in 0..maze.columns {
                        maze.draw_cell(x, y, &mut buffer, WIDTH, HEIGHT);
                    }
                }
                maze.solve(&mut buffer, WIDTH, HEIGHT, &mut window);
                is_paused = true;
            } else {
                let cell_width = 16;
                let cell_height = 18;
                let start = Point::new(1, 1);
                buffer.fill(0x333333);
                let mut maze = Maze::new(start, 39, 79, cell_width, cell_height);
                maze.break_walls(WIDTH, HEIGHT);
                maze.reset_visited_cells();
                for y in 0..maze.rows {
                    for x in 0..maze.columns {
                        maze.draw_cell(x, y, &mut buffer, WIDTH, HEIGHT);
                    }
                }
                maze.solve(&mut buffer, WIDTH, HEIGHT, &mut window);
                is_paused = true;
            }
        }
        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
    }
}
