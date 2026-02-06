use rand::Rng;
use std::io;

fn display() {
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
            accept_guesses(&number);
        }
        Err(_) => {
            let result = 0;
            accept_guesses(&result);
        }
    }
}

fn accept_guesses(&guess: &i32) {
    match guess {
        1 => {
            println!("Playing Game");
            play()
        },
        2 => {
            help()
        },
        0 => exit(),
        _ => todo!()
    }
}

fn generate_random_number() -> i32 {
    let mut rng = rand::rng();
    let sercret_number = rng.random_range(1..=100);

    sercret_number
}

fn play() {
    let mut tries_count = 0;
    let mut input = String::new();
    let mut _refined_input = 0;

    let random_generated_number = generate_random_number();

    loop {
        input.clear();

        println!("Guess a number between 1-100:");

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");

        let _refined_input = match input.trim().parse::<i32>() {
            Ok(num) => {
                if num < 1 || num > 100 {
                    println!("");
                    println!("=====================================");
                    println!("Number must be between 1 through 100");
                    println!("=====================================");
                    println!("");

                    continue;
                }

                num
            },
            Err(_) => {
                println!("Please enter a valid number.");
                continue;
            }
        };

        tries_count += 1;

        if _refined_input == random_generated_number {
            println!("You go it!");
            println!("Number of tries: {}", tries_count);

            break;
        }
        if _refined_input > random_generated_number {
            println!("High than secret number");
            continue;
        }
        if _refined_input < random_generated_number {
            println!("Lower than secret number");
            continue;
        }
    }
}

fn help() {
}

fn exit() {
    println!("Exiting...");
}

fn main() {
    display();
}
