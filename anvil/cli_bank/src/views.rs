use std::io;

use crate::account;

use super::services::*;
use super::utilities::*;
use super::bank::*;

pub fn hello() {
    clear_screen();

    println!("\t================================================================");
    println!("\tWelcome To The Number 1 Trusted Bank In The Entire Universe of Banks");
    println!("\t================================================================");
    println!();


}

pub fn home() -> i32 {
    let mut input = String::new();

    println!("\t1. Login");
    println!("\t2. Help");
    println!("\t0. Exit");

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    let int_input = parse(input);

    match int_input {
        1 => { login(); },
        2 => { help(); },
        0 => { exit(); },
        _ => {
            println!("\tPlease enter a valid input in this range [0 - 2]");
            redirect_to_home();
        }
    }

    todo!()
}

pub fn login() {
    clear_screen();

    let mut login_input = String::new();
    println!("\tWhat is your name?");

    io::stdin()
        .read_line(&mut login_input)
        .expect("Failed to read input");

    let name = login_input.trim();

    if !name.is_empty() {
        let user = create_user(name.to_string());
        greet_user(&user.name);
        menu();
    } else {
        println!("Failed to Login: Name cannot be empty");
        wait(1);
        println!("Redirecting back to home");
        redirect_to_home();
    }
}

pub fn greet_user(name: &str) {
    println!("Hello, {}", name);
}

pub fn menu() {
    clear_screen();

    println!("\t================================================================");
    println!("\tWelcome To The Number 1 Trusted Bank In The Entire Universe of Banks");
    println!("\t================================================================");

    println!("\t** MENU **");
    println!("\t1. Create Bank Account");
    println!("\t2. Deposit");
    println!("\t3. Withdraw");
    println!("\t4. Get Balance");
    println!("\t0. Exit");

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    let input = input.trim();

    match input {
        "1" => {
            let account = Bank::new();
         },
        "2" => {
            clear_screen();
            println!("How much would you like to deposit");

            let mut input = String::new();
            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read input");

            let amount = parse_into_float(input);

            Bank::deposit(account_id, amount);
         },
        "3" => {  },
        "4" => {  },
        "0" => {  },
        _ => {
            println!("\tPlease enter a valid input in this range [0 - 2]");
            redirect_to_home();
        }
    }
}

fn help() {
    clear_screen();
    println!("\t================================================================");
    println!("\tWelcome To The Number 1 Trusted Bank In The Entire Universe of Banks ");
    println!("\t================================================================");
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
    println!("Press Enter to return to the main home...");

    // Wait for the user to press Enter
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    clear_screen();
    redirect_to_home();
}

pub fn redirect_to_home() {
    wait(2);
    home();
}

pub fn redirect_to_menu() {
    wait(2);
    menu();
}
