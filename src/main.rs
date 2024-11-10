mod line;
mod point;

use minifb::{Key, KeyRepeat, Window, WindowOptions};
use point::Point;

const WIDTH: usize = 1200;
const HEIGHT: usize = 800;
fn main() {
    let mut window = Window::new("Maze Solver", WIDTH, HEIGHT, WindowOptions::default())
        .unwrap_or_else(|e| panic!("Failed to create window: {}", e));

    let mut buffer: Vec<u32> = vec![0x0; WIDTH * HEIGHT];
    let mut update_buffer = false;
    while window.is_open() && !window.is_key_down(Key::Escape) {
        if window.is_key_pressed(Key::Space, KeyRepeat::No) {
            update_buffer = !update_buffer;
        }
        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
    }
}
