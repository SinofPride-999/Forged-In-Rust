use std::{thread, time};

/// parse string input from the terminal to i32
pub fn parse(input: String) -> i32 {
    match input.trim().parse::<i32>() {
        Ok(number) => {
            number
        }
        Err(_) => {
            0
        }
    }
}

pub fn exit() {
    println!("Exiting...");
    wait(3);

    println!("Goodbye");
    wait(1);
    clear_screen();
}

pub fn wait(secs: u64) {
    let duration = time::Duration::from_secs(secs);
    thread::sleep(duration);
}

pub fn clear_screen() {
    // \x1B is the ESC character
    // [2J clears the screen, [1;1H moves cursor to top-left
    print!("\x1B[2J\x1B[1;1H");
}

