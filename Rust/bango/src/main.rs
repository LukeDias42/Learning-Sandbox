use shuffle::irs::Irs;
use shuffle::shuffler::Shuffler;
use std::io;

fn main() {
    let mut rng = rand::thread_rng();
    let mut irs = Irs::<u8>::default();

    let mut input = Vec::new();
    for i in 1..=75 {
        input.push(i);
    }

    _ = irs.shuffle(&mut input, &mut rng);

    let mut enter = String::new();
    let reader = io::stdin();
    let mut drawn_numbers = Vec::new();
    for value in input {
        drawn_numbers.push(value);
        print_numbers(&drawn_numbers, value);
        reader.read_line(&mut enter).expect("Error");
    }
}

fn print_numbers(drawn_numbers: &[u8], last_drawn: u8) {
    print!("\x1B[2J\x1B[1;1H");
    for i in 1..=75 {
        if drawn_numbers.contains(&(i as u8)) {
            print!("\x1b[1;32m");
            if i < 10 {
                print!("0{} ", i);
            } else {
                print!("{} ", i);
            }
            print!("\x1b[1;39m");
        } else {
            print!("\x1b[1;31m");
            print!("__ ");
            print!("\x1b[1;39m");
        }

        if i % 5 == 0 {
            println!();
        }
        if i % 15 == 0 {
            println!();
        }
    }
    println!("Número Sorteado: {last_drawn}")
}
