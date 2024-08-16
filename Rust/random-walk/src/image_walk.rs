use super::move_square::update_position;
use super::splitmix::SplitMix;
use image::{Rgb, RgbImage};
use stopwatch::Stopwatch;

pub fn random_image(
    file_name: &str,
    width: u32,
    height: u32,
    repetitions: u64,
    square_size: isize,
    mut color_change_speed: i64,
) {
    let mut img = RgbImage::from_fn(width, height, |_, _| Rgb([0x0, 0x0, 0x0]));

    let mut hasher = SplitMix::new();
    let mut pos = (
        hasher.rand(width as u64) as isize,
        hasher.rand(height as u64) as isize,
    );
    let mut color_value: i64 = 0xff_ff_ff;
    println!("{repetitions} movements started.");
    let sw = Stopwatch::start_new();

    for _ in 0..repetitions {
        let c = Rgb([
            (color_value >> 0x08 & 0xff) as u8,
            (color_value >> 0x04 & 0xff) as u8,
            (color_value & 0xff) as u8,
        ]);
        if color_value <= 0 || color_value >= 0xff_ff_ff {
            color_change_speed *= -1i64;
        }
        color_value += color_change_speed;
        for i in 0..square_size as u32 {
            for j in 0..square_size as u32 {
                img.put_pixel(pos.0 as u32 + i, pos.1 as u32 + j, c);
            }
        }
        pos = update_position(&mut hasher, pos, width, height, square_size);
    }

    println!("{} ms to complete.", sw.elapsed_ms());
    img.save(format!("{file_name}.png"))
        .unwrap_or_else(|err| println!("An error occured while trying to save the image: {err}"));
    println!("Saved image: {file_name}.png.")
}
