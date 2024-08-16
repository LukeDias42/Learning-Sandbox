use super::move_square::update_position;
use super::splitmix::SplitMix;
use minifb::{Key, Window, WindowOptions};

pub fn random_animation(width: usize, height: usize) {
    let mut window = Window::new("Random Walk", width, height, WindowOptions::default())
        .unwrap_or_else(|e| {
            panic!("{}", e);
        });

    let mut buffer = vec![0x0; width * height];
    let mut sm_pos = SplitMix::build(1234567);
    let mut pos = (width as isize / 2, height as isize / 2);
    let square_size = 1;
    let mut color = 0xff_ff_ff;
    let mut c_acc: i32 = -0xf;
    while window.is_open() && !window.is_key_down(Key::Escape) {
        let x = pos.0.clamp(0, width as isize) as usize;
        let y = pos.1.clamp(0, height as isize) as usize;
        for i in 0..square_size {
            for j in 0..square_size {
                buffer[(x + i as usize) + (y + j as usize) * width] = color as u32;
            }
        }
        color += c_acc;
        if color <= 0 || color >= 0xffffff {
            c_acc *= -1;
        }
        window.update_with_buffer(&buffer, width, width).unwrap();
        pos = update_position(&mut sm_pos, pos, width as u32, width as u32, square_size);
    }
}
