mod animation_walk;
mod image_walk;
mod move_square;
mod splitmix;
use animation_walk::random_animation;
use image_walk::random_image;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let random_walk_type = args.get(1).and_then(|s| s.parse().ok()).unwrap_or(0);
    if random_walk_type == 0 {
        random_image("test", 3920, 2160, 0x00_FFFFFF, 5, 1);
    } else {
        random_animation(600, 600);
    }
}
