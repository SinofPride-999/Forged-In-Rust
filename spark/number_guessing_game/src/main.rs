use rand::Rng;
use std::io;

fn _generate_random_number() -> i32 {
    let mut rng = rand::rng();
    let sercret_number = rng.random_range(1..=100);

    sercret_number
}

fn display() -> i32 {
    println!("=====================");
    println!("Number Guessing Game");
    println!("=====================");

    println!("");

    let mut input = String::new();

    println!("Enter [1, 2, 0]: ");
    println!("1. Play");
    println!("2. Help");
    println!("0. Exit");

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    match input.trim().parse::<i32>() {
        Ok(number) => {
            println!("Parsed number: {}", number);
            number
        }
        Err(e) => {
            println!("Failed to parse number: {}", e);
            0
        }
    }
}

fn main() {
    display();
}
