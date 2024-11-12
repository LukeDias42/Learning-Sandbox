mod cell;
mod color;
mod line;
mod maze;
mod point;

use core::panic;
use maze::Maze;
use minifb::{Key, KeyRepeat, Window, WindowOptions};
use point::Point;

const WIDTH: usize = 1200;
const HEIGHT: usize = 800;
fn main() {
    let mut window = Window::new("Maze Solver", WIDTH, HEIGHT, WindowOptions::default())
        .unwrap_or_else(|e| panic!("Failed to create window: {}", e));

    let mut buffer: Vec<u32> = vec![0x0; WIDTH * HEIGHT];
    let start = Point::new(100, 100);
    while window.is_open() && !window.is_key_down(Key::Escape) {
        if window.is_key_pressed(Key::Space, KeyRepeat::No) {
            buffer.fill(0x333333);
            let mut maze = Maze::new(start, 10, 15, 50, 50);
            maze.break_walls_r(0, 0, WIDTH, HEIGHT);
            maze.reset_visited_cells();
            for y in 0..maze.rows {
                for x in 0..maze.columns {
                    maze.draw_cell(x, y, &mut buffer, WIDTH, HEIGHT);
                }
            }
            maze.solve(&mut buffer, WIDTH, HEIGHT, &mut window);
        }
        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
    }
}
