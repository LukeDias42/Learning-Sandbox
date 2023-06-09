use minifb::{Key, Window, WindowOptions};
use image::{ RgbImage, Rgb };
use std::time::{SystemTime, UNIX_EPOCH};

fn main() {
    random_image();
    // random_animation();
}

fn random_image() {
    const WIDTH: usize = 3940;
    const HEIGHT: usize = 2160;
    let mut img = RgbImage::from_fn(
        WIDTH as u32, 
        HEIGHT as u32, 
        |_,_| Rgb([0x0, 0x0, 0x0])
    );

    let mut sm_pos = SplitMix::new();
    let mut pos = (sm_pos.rand(WIDTH as u64) as isize, 
                   sm_pos.rand(HEIGHT as u64) as isize);
    let mut color = 0xff_ff_ff;
    let mut c_add = -0x1;
    let mut l_acc: u64 = 0;
    let size = 1;
    loop {
        let x = pos.0.clamp(0, WIDTH as isize - size) as u32;
        let y = pos.1.clamp(0, HEIGHT as isize - size) as u32;
        let c = Rgb([
            (color >> 0x10 & 0xff) as u8, 
            ((color >> 0x10 & 0xff) >> 7) as u8, 
            ((color >> 0x08 & 0xff) >> 2) as u8]);
        for i in 0..size {
            for j in 0..size {
                img.put_pixel(x + i as u32, y + j as u32, c);
            }
        }
        color += c_add;
        if color <= 0 || color >= 0xff_ff_ff { c_add *= -1; }
        pos = update_position(&mut sm_pos, pos, WIDTH, HEIGHT, 1);
        l_acc +=1;
        if l_acc >= 0x0fff_ffff { break; }
    }
    img.save("random_walk_7.png").unwrap_or_else(|err| {
        println!("An error occured while trying to save the image: {err}");
        return;
    });
    println!("saved image.");
}

fn random_animation() {
    const WIDTH: usize = 960;
    const HEIGHT: usize = 540;
    let mut window = Window::new("Random Walk", WIDTH, HEIGHT, WindowOptions::default())
        .unwrap_or_else(|e| {
            panic!("{}", e);
        });

    let mut buffer = [0x0; WIDTH * HEIGHT];
    let mut sm_pos = SplitMix::build(1234567);
    let mut pos = (WIDTH as isize / 2, HEIGHT as isize / 2);
    let square_size = 1;
    let mut color = 0xff_ff_ff;
    let mut c_acc: i32 = -0xf;
    while window.is_open() && !window.is_key_down(Key::Escape) {
        let x = pos.0.clamp(0, WIDTH as isize) as usize;
        let y = pos.1.clamp(0, HEIGHT as isize) as usize;
        for i in 0..square_size {
            for j in 0..square_size {
                buffer[(x + i as usize) + (y + j as usize) * WIDTH] = color as u32;
            }
        }
        color += c_acc;
        if color <= 0 || color >= 0xffffff { c_acc *= -1; }
        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
        pos = update_position(&mut sm_pos, pos, WIDTH, HEIGHT, square_size);
    }
}

fn update_position(
    sm: &mut SplitMix, 
    pos: (isize, isize), 
    width: usize, height: usize, 
    square_size: isize) 
    -> (isize, isize) {
    let mut n = sm.rand(8);
    let mut new_pos = (pos.0, pos.1);
    n = if n >= 4 { n + 1} else { n };
    new_pos.0 += (((n % 3) as isize - 1) as isize) * square_size;
    new_pos.1 += (((n / 3) as isize - 1) as isize) * square_size;
    new_pos.0 = rotate(new_pos.0,  square_size, width as isize);
    new_pos.1 = rotate(new_pos.1,  square_size, height as isize);
    new_pos
}

fn rotate(value: isize, square_size: isize, length: isize) -> isize {
    if value + square_size >= length {
        0
    } else if value < 0 {
        length as isize - square_size
    } else {
        value
    }
}

pub struct SplitMix {
    state: u64,
}

impl SplitMix {
    pub fn new() -> SplitMix {
        let since_the_epoch = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Clock may have gone backwards")
            .as_millis();
        SplitMix {
            state: since_the_epoch as u64,
        }
    }

    pub fn build(initial_state: u64) -> SplitMix {
        SplitMix {
            state: initial_state,
        }
    }

    pub fn next(&mut self) -> u64 {
        self.state = self.state.wrapping_add(0x9e3779b97f4a7c15);
        let mut z = self.state;
        z = (z ^ (z >> 30)).wrapping_mul(0xbf58476d1ce4e5b9_u64);
        z = (z ^ (z >> 27)).wrapping_mul(0x94d049bb133111eb_u64);
        z ^ (z >> 31)
    }

    pub fn next_float(&mut self) -> f64 {
        (self.next() as f64) / 2_f64.powi(64)
    }

    pub fn rand(&mut self, max: u64) -> u64 {
        (self.next_float() * max as f64) as u64
    }
}
