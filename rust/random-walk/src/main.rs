mod image_walk;
mod animation_walk;
mod splitmix;
mod move_square;
use image_walk::random_image;
use animation_walk::random_animation;

fn main() {
    random_image(
        "test",      // filename
        3920,        // width 
        2160,        // height
        0x00_FFFFFF, // repetition
        5,           // square size
        1);          // color change speed
    // random_animation(600, 600);
}




