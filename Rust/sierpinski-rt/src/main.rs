use minifb::{Key, Window, WindowOptions};
use std::{thread, time};

const SIZE: usize = 749;

fn main() {
    let mut buffer: Vec<u32> = vec![0x001A4D; SIZE * SIZE];
    // Create a window to display the image
    let mut window = Window::new(
        "Sierpinski Carpet",
        SIZE,
        SIZE,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });

    let mut steps = 0;
    let mut side = SIZE;
    while side > 1 { steps += 1; side /= 3; }
    steps -= 1;

    window.update_with_buffer(&buffer, SIZE, SIZE).unwrap();
    while window.is_open() && !window.is_key_down(Key::Enter) {
        window.update();
    }
    for step in 0..=steps {
        sierpinski_carpet(&mut buffer, step, SIZE as u32, &mut window);
        buffer = vec![0x001A4D; SIZE * SIZE];
        thread::sleep(time::Duration::from_millis(1000));
    }
    while window.is_open() && !window.is_key_down(Key::Escape) {
        window.update();
    }
}


fn sierpinski_carpet(buffer: &mut Vec<u32>, steps: u32, side: u32, window: &mut Window) {
    let sqr = Square { x0: 0, y0: 0, len: side };
    let mut queue = vec![(sqr, steps)];

    while !queue.is_empty() {
        let (sqr, steps) = queue.remove(0);
        fill_center_square(&sqr, buffer);
        window.update_with_buffer(&buffer, SIZE, SIZE).unwrap();
        if steps == 0 { 
            continue; 
        }

        let new_len = sqr.len/3;
        for i in 0..=2 {
            for j in 0..=2 {
                if i == 1 && j ==1 { continue; }
                queue.push(
                    (Square::new(
                        sqr.x0 + i*new_len, 
                        sqr.y0 + j*new_len, 
                        new_len), 
                    steps - 1));
            }
        }
    }
}

fn fill_center_square(sqr: &Square, buffer: &mut Vec<u32>) {
    let inner_len = sqr.len/3;
    for x in 0..inner_len {
        for y in 0..inner_len {
            let column = inner_len + sqr.x0 + x;
            let row = inner_len + sqr.y0 + y;
            buffer[(column + row * SIZE as u32) as usize] = 0xFFD700 as u32;
        }
    }
}

struct Square {
    x0: u32,
    y0: u32,
    len: u32
}

impl Square {
    fn new(x0: u32, y0: u32, len: u32) -> Square {
        Square { x0, y0, len }
    }
}
