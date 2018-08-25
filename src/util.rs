use std::process::Command;
use std::thread;
use std::time::Duration;
use std::io::{self, Write};
use super::game::{Input};

pub fn clear_terminal() {
    Command::new("clear").status()
        .expect("Failed to clear console");
}

pub fn sleep(ms: usize) {
    let ms = ms as u64;
    thread::sleep(Duration::from_millis(ms));
}

pub fn display_intro(seconds: usize) {
    println!("----------------------");
    println!("Welcome to Tic Tac Toe");
    println!("----------------------");
    println!();
    println!("    Hotseat Mode");
    println!();

    for x in 0..seconds {
        println!("Game will start in {} seconds...", seconds - x);
        sleep(1000);
    }
}

pub fn get_user_input() -> Input {
    let mut row_col = (-1, -1);

    loop {
        println!();
        print!("Input your row and col separated by space (e.g, 2 2): ");
        let _ = io::stdout().flush();

        let mut buffer = String::new();
        io::stdin()
            .read_line(&mut buffer)
            .expect("Failed to read user input");

        let input_str = buffer
            .trim()
            .to_string();

        // Validate user input
        let input_vec: Vec<_> = input_str
            .split_whitespace()
            .collect();

        if input_vec.len() == 2 {
            if let Ok(row) = input_vec[0].parse() {
                row_col.0 = row;

                if let Ok(col) = input_vec[1].parse() {
                    row_col.1 = col;

                    let row = row_col.0;
                    let col = row_col.1;

                    if row > 2 || col > 2 {
                        println!("Row and col must be an integer between 0 to 2");
                        continue;
                    } else {
                        return Input { row, col }
                    }
                } else {
                    println!("Col must be an integer between 0 to 2");
                    continue;
                }
            } else {
                println!("Row must be an integer between 0 to 2");
                continue;
            }
        } else {
            continue;
        }
    }
}

pub fn display_outro(game_board: String, winning_player: String) {
    println!("----------------------");
    println!("      Game Over       ");
    println!("----------------------");
    println!();
    println!("{}", game_board);
    println!();
    println!("Player {} wins! Congratulations!", winning_player);
}
