use super::splitmix::SplitMix;

pub fn update_position(
    hasher: &mut SplitMix,
    pos: (isize, isize),
    width: u32,
    height: u32,
    square_size: isize,
) -> (isize, isize) {
    let mut n = hasher.rand(8);
    let mut new_pos = (pos.0, pos.1);
    n = if n >= 4 { n + 1 } else { n };
    new_pos.0 += ((n % 3) as isize - 1) * square_size;
    new_pos.1 += ((n / 3) as isize - 1) * square_size;
    new_pos.0 = rotate(new_pos.0, square_size, width as isize);
    new_pos.1 = rotate(new_pos.1, square_size, height as isize);
    new_pos
}

fn rotate(value: isize, square_size: isize, length: isize) -> isize {
    if value + square_size >= length {
        0
    } else if value < 0 {
        length - square_size
    } else {
        value
    }
}
