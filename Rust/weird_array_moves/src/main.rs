use std::env;

fn main() {
    let (columns, rows) = get_sizes();
    print_zig_zag_array(columns, rows);
}

fn get_sizes() -> (usize, usize) {
    let args: Vec<String> = env::args().collect();
    let columns = args.get(1).and_then(|s| s.parse().ok()).unwrap_or(5);
    let lines = args.get(2).and_then(|s| s.parse().ok()).unwrap_or(5);
    (columns, lines)
}

fn print_zig_zag_array(columns: usize, rows: usize) {
    let digits = calculate_digits(columns * rows);
    let r = rows as isize;
    for y in 0..rows {
        for x in 0..columns {
            print! {"{:>1$}", calculate_zig_zag_value(x as isize, y as isize, r), digits};
        }
        println!();
    }
}

fn calculate_digits(mut value: usize) -> usize {
    let mut digits = 1;
    while value > 0 {
        digits += 1;
        value /= 10;
    }
    digits
}

fn calculate_zig_zag_value(x: isize, y: isize, rows: isize) -> isize {
    x * rows + y + 1 + x % 2 * (rows - 2 * y - 1)
}
