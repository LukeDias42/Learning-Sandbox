use core::{
    f64,
    ops::{Add, Mul},
};
use image::{Rgb, RgbImage};
use split_mix::SplitMix;

pub fn generate_perlin_image(noise_map: NoiseMap, file_name: &str) {
    let per = PermutationTable::new_from(124);
    let (width, height) = noise_map.size;
    let x_bounds = noise_map.x_bound;
    let y_bounds = noise_map.y_bound;
    let mut map = vec![0.0; width * height];

    let x_extent = x_bounds.1 - x_bounds.0;
    let y_extent = y_bounds.1 - y_bounds.0;

    let x_step = x_extent / width as f64;
    let y_step = y_extent / height as f64;

    for y in 0..height {
        let current_y = y_bounds.0 + y_step * y as f64;
        for x in 0..width {
            let current_x = x_bounds.0 + x_step * x as f64;
            map[x + y * width] = perlin_2d((current_x, current_y), &per);
        }
    }
    let mut pixels: Vec<u8> = Vec::with_capacity(width * height);
    for i in map {
        pixels.push(((i * 0.5 + 0.5).clamp(0.0, 1.0) * 255.0) as u8);
    }
    let image = RgbImage::from_fn(width as u32, height as u32, |x, y| {
        let shade = pixels[(x + y * width as u32) as usize];
        Rgb([shade, shade, shade])
    });

    image.save(file_name).unwrap_or_else(|err| {
        println!("An error occured while trying to save the image: {err}");
    });
}

pub fn perlin_2d(point: (f64, f64), hasher: &PermutationTable) -> f64 {
    const SCALE_FACTOR: f64 = 2.0 / f64::consts::SQRT_2;

    let corner = (point.0.floor() as isize, point.1.floor() as isize);
    let distance = (point.0 - corner.0 as f64, point.1 - corner.1 as f64);

    let g00 = gradient((0, 0), distance, corner, hasher);
    let g01 = gradient((0, 1), distance, corner, hasher);
    let g10 = gradient((1, 0), distance, corner, hasher);
    let g11 = gradient((1, 1), distance, corner, hasher);

    let curve = (fade(distance.0), fade(distance.1));

    let result = lerp(lerp(g00, g01, curve.1), lerp(g10, g11, curve.1), curve.0) * SCALE_FACTOR;

    result.clamp(-1.0, 1.0)
}

fn gradient(
    offset: (isize, isize),
    distance: (f64, f64),
    corner: (isize, isize),
    hasher: &PermutationTable,
) -> f64 {
    let point = (distance.0 - offset.0 as f64, distance.1 - offset.1 as f64);
    let to_hash = [corner.0 + offset.0, corner.1 + offset.1];
    let hash = hasher.hash(&to_hash);

    match hash & 0b11 {
        0 => point.0 + point.1,  // ( 1,  1)
        1 => -point.0 + point.1, // (-1,  1)
        2 => point.0 - point.1,  // ( 1, -1)
        3 => -point.0 - point.1, // (-1, -1)
        _ => unreachable!(),
    }
}

fn fade(t: f64) -> f64 {
    t * t * t * (10f64 + t * (-15f64 + 6f64 * t))
}

fn lerp<T>(a: T, b: T, alpha: f64) -> T
where
    T: Mul<f64, Output = T> + Add<Output = T>,
{
    b * alpha + a * (1_f64 - alpha)
}

pub struct NoiseMap {
    size: (usize, usize),
    x_bound: (f64, f64),
    y_bound: (f64, f64),
}

impl NoiseMap {
    pub fn new(size: (usize, usize), x_bound: (f64, f64), y_bound: (f64, f64)) -> Self {
        Self {
            size,
            x_bound,
            y_bound,
        }
    }
}

#[derive(Debug)]
pub struct PermutationTable {
    values: Vec<u8>,
}

impl PermutationTable {
    pub fn new_from(seed: u64) -> Self {
        let mut sm = SplitMix::new_from(seed);
        let mut permutation: Vec<u64> = (0..=255).collect();
        sm.shuffle_vec(&mut permutation);
        let mut pt = PermutationTable { values: Vec::new() };
        for i in 0..256 {
            pt.values.push(permutation[i] as u8)
        }
        pt
    }

    pub fn new() -> Self {
        let mut sm = SplitMix::new();
        let mut permutation: Vec<u64> = (0..=255).collect();
        sm.shuffle_vec(&mut permutation);
        let mut pt = PermutationTable { values: Vec::new() };
        for i in 0..256 {
            pt.values.push(permutation[i] as u8)
        }
        pt
    }

    pub fn hash(&self, to_hash: &[isize]) -> usize {
        let index = to_hash
            .iter()
            .map(|&a| (a & 0xff) as usize)
            .reduce(|a, b| self.values[a] as usize ^ b)
            .unwrap();
        self.values[index] as usize
    }
}
