use perlin::{ PermutationTable, NoiseMap };
use minifb::{Key, Window, WindowOptions};

fn main() {
    // perlin::generate_perlin_image(
    // NoiseMap::new((1280, 720), (-16.0, 16.0), (-9.0, 9.0)), 
    // "perlin.png"
    // );
    show_animation();
}

fn show_animation() {
    const SIZE: usize = 720;

    let hasher = PermutationTable::new();
    // Create a window to display the image
    let mut window = Window::new(
        "Perlin Square",
        SIZE,
        SIZE,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });

    let square_size = 15;
    let mut counter = 0.0;
    while window.is_open() && !window.is_key_down(Key::Escape) {
        let mut buffer: Vec<u32> = vec![0x0; SIZE * SIZE];
        let x_perlin = perlin::perlin_2d((counter, counter + 1000.0), &hasher);
        let y_perlin = perlin::perlin_2d((counter + 1000.0, counter), &hasher);
        let x_pos = (x_perlin * 0.5 + 0.5).clamp(0.0, 1.0);
        let y_pos = (y_perlin * 0.5 + 0.5).clamp(0.0, 1.0);
        let x = ((x_pos * (SIZE) as f64) as usize).clamp(square_size, SIZE - square_size);
        let y = ((y_pos * (SIZE) as f64) as usize).clamp(square_size, SIZE - square_size);
        for i in 0..square_size {
            for j in 0..square_size {
                buffer[(x - i) + (y - j) * SIZE] = 0xffffff;
            }
        }
        window.update_with_buffer(&buffer, SIZE, SIZE).unwrap();
        counter += 1.0 / SIZE as f64;
    }

}
