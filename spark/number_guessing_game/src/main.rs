use rand::Rng;
use std::{io, thread, time};

fn display() {
    clear_screen();
    println!("=====================");
    println!("Number Guessing Game");
    println!("=====================");

    println!("");

    let mut input = String::new();

    println!("Enter [1, 2, 0]: ");
    println!("1. Play");
    println!("2. Help");
    println!("0. Exit");
    println!("");
    println!("~>");

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

fn accept_guesses(guess: &i32) {
    match guess {
        1 => {
            println!("Loading Game ... Please wait");
            wait(2);

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
    clear_screen();

    let mut tries_count = 0;
    let mut input = String::new();
    let mut _refined_input = 0;

    let random_generated_number = generate_random_number();

    loop {
        input.clear();

        println!("Guess a number between 1-100:");
        println!("");
        println!("~>");

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
            println!("You got it!");
            wait(2);
            println!("Number of tries: {}", tries_count);

            let mut winning_input = String::new();

            println!("Would you like to play again?");

            io::stdin()
                .read_line(&mut winning_input)
                .expect("Error reading line");

            let winning_input = winning_input.trim().to_string();
            if winning_input == "y" {
                play();
            } else if winning_input == "n" {
                exit();
            }

            break;
        }
        if _refined_input > random_generated_number {
            println!("Higher than secret number");
            continue;
        }
        if _refined_input < random_generated_number {
            println!("Lower than secret number");
            continue;
        }
    }
}

fn help() {
    clear_screen();
    println!("=====================");
    println!("      HELP MENU      ");
    println!("=====================");
    println!();
    println!("Welcome to the Number Guessing Game!");
    println!("Here’s how to play:");
    println!();
    println!("1. The computer will generate a random number between 1 and 100.");
    println!("2. Your goal is to guess the number.");
    println!("3. After each guess, you'll get a hint:");
    println!("   - 'Higher than secret number' means your guess is too high.");
    println!("   - 'Lower than secret number' means your guess is too low.");
    println!("4. Keep guessing until you find the correct number.");
    println!("5. You can only enter numbers between 1 and 100.");
    println!("6. Invalid inputs (letters, symbols, or numbers out of range) will be rejected.");
    println!();
    println!("Press Enter to return to the main menu...");

    // Wait for the user to press Enter
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    clear_screen();
    display();
}

fn exit() {
    println!("Exiting...");
    wait(3);

    println!("Goodbye");
    wait(1);
    clear_screen();
}

fn wait(secs: u64) {
    let duration = time::Duration::from_secs(secs);
    thread::sleep(duration);
}

fn clear_screen() {
    // \x1B is the ESC character
    // [2J clears the screen, [1;1H moves cursor to top-left
    print!("\x1B[2J\x1B[1;1H");
}

fn main() {
    display();
}
