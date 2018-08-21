extern crate tictactoe;

use std::process::Command;
use std::io::{self, Write};
use std::thread;
use std::time::Duration;

use tictactoe::{TicTacToe};

fn main() {
    play_hotseat();
}

fn play_hotseat() {
    clear_terminal();
    let mut game = TicTacToe::new();

    println!("----------------------");
    println!("Welcome to Tic Tac Toe");
    println!("----------------------");
    println!();
    println!("Game will start in 3 seconds...");

    thread::sleep(Duration::from_secs(3));

    loop {
        clear_terminal();

        println!();
        println!("{}", game.get_board());
        println!();
        println!("Turn:  Player {}", game.get_turn());
        println!();

        // Ask user input
        let mut row_col = (-1, -1);
        loop {
            println!();
            print!("Input your row and col separated by space: ");
            io::stdout().flush();

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
                        let turn = game.get_turn();

                        println!();
                        println!(
                            "Player {} chose row: {}, col: {}", 
                            turn,
                            row,
                            col,
                        );
                        println!();
            
                        match game.fill(row, col) {
                            Ok(_) => break,
                            Err(msg) => {
                                println!("Invalid move, {}", msg);
                                continue;
                            }
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

        if game.check_game_over() {
            break;
        } else {
            game.next_turn();
        }
    }

    clear_terminal();

    println!("----------------------");
    println!("      Game Over       ");
    println!("----------------------");
    println!();
    println!("{}", game.get_board());
    println!();
    println!("Player {} wins! Congratulations!", game.get_turn());
}

fn clear_terminal() {
    Command::new("clear").status()
        .expect("Failed to clear console");
}
