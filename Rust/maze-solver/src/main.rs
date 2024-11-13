mod cell;
mod cell_hex;
mod color;
mod line;
mod maze;
mod maze_hex;
mod point;

use color::Color;
use core::panic;
use maze::Maze;
use maze_hex::MazeHex;
use minifb::{Key, KeyRepeat, Window, WindowOptions};
use point::Point;

const WINDOW_WIDTH: usize = 1280;
const WINDOW_HEIGHT: usize = 720;
const TARGET_FPS: usize = 144;

// Standard cells
const CELL_WIDTH: usize = 16;
const CELL_HEIGHT: usize = 18;
const CELL_ROWS: usize = 39;
const CELL_COLUMNS: usize = 79;

// #333333
const BACKGROUND_COLOR: Color = Color {
    r: 0x33,
    g: 0x33,
    b: 0x33,
};

// Hexagonal cells
const HEX_CELL_LEN: usize = 20;
const HEX_CELL_COLUMNS: usize = 41;
const HEX_CELL_ROWS: usize = 20;
// #2B2D42
const HEX_BACKGROUND_COLOR: Color = Color {
    r: 0x2B,
    g: 0x2D,
    b: 0x42,
};

fn main() {
    let mut window = Window::new(
        "Maze Solver",
        WINDOW_WIDTH,
        WINDOW_HEIGHT,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| panic!("Failed to create window: {}", e));
    window.set_target_fps(TARGET_FPS);
    let mut buffer: Vec<u32> = vec![0; WINDOW_WIDTH * WINDOW_HEIGHT];
    demo_loop(&mut buffer, &mut window);
}

fn demo_loop(mut buffer: &mut Vec<u32>, mut window: &mut Window) {
    let mut is_hex = true;
    let mut is_paused = true;
    while window.is_open() && !window.is_key_down(Key::Escape) {
        {
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
                    let start = Point::new(HEX_CELL_LEN, HEX_CELL_LEN);
                    MazeHex::animate(
                        start,
                        HEX_CELL_ROWS,
                        HEX_CELL_COLUMNS,
                        HEX_CELL_LEN,
                        HEX_BACKGROUND_COLOR,
                        WINDOW_WIDTH,
                        WINDOW_HEIGHT,
                        &mut buffer,
                        &mut window,
                    );
                    is_paused = true;
                } else {
                    let start = Point::new(1, 1);
                    Maze::animate(
                        start,
                        CELL_ROWS,
                        CELL_COLUMNS,
                        CELL_WIDTH,
                        CELL_HEIGHT,
                        BACKGROUND_COLOR,
                        WINDOW_WIDTH,
                        WINDOW_HEIGHT,
                        &mut buffer,
                        &mut window,
                    );
                    is_paused = true;
                }
            }
            window
                .update_with_buffer(&buffer, WINDOW_WIDTH, WINDOW_HEIGHT)
                .unwrap();
        }
    }
}
