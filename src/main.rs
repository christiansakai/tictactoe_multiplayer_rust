extern crate tictactoe;

use std::env;
use std::io::{self, Write};

use tictactoe::game::{TicTacToe};
use tictactoe::network::{server, client};
use tictactoe::util;

fn main() {
    util::clear_terminal();

    let args: Vec<String> = env::args().collect();

    if args.len() > 1 && args[1] == "server" {
        println!("Initialize as Server");
        println!("Not implemented yet");
        // server::listen();
    } else if args.len() > 1 && args[1] == "client" {
        println!("Initialize as Client");
        println!("Not implemented yet");
        // client::connect();
    } else {
        play_hotseat();
    }

}

// Pseudo code
// fn play_lan() {
//     util::clear_terminal();

//     // Server
//     let mut game = TicTacToe::new();
//     let server = server::listen();

//     loop {
//         let turn = game.get_turn();
//         server.send_message_to_client_to_tell_turn(turn);
//         server.on_receive_message_from_client(|message| {
//             let player = message.player;
//             let row = message.row;
//             let col = message.col;

//             game.fill(row, col);
//         });

//         if game.check_game_over() {
//             break;
//             server.send_message_to_client_to_tell_result();
//         } else {
//             let _ = game.next_turn();
//         }
//     }
// }

fn play_hotseat() {
    util::clear_terminal();
    let mut game = TicTacToe::new();

    println!("----------------------");
    println!("Welcome to Tic Tac Toe");
    println!("----------------------");
    println!();
    println!("    Hotseat Mode");
    println!();

    println!("Game will start in 3 seconds...");
    util::sleep(1000);
    println!("Game will start in 2 seconds...");
    util::sleep(1000);
    println!("Game will start in 1 seconds...");
    util::sleep(1000);

    loop {
        util::clear_terminal();

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
            let _ = game.next_turn();
        }
    }

    util::clear_terminal();

    println!("----------------------");
    println!("      Game Over       ");
    println!("----------------------");
    println!();
    println!("{}", game.get_board());
    println!();
    println!("Player {} wins! Congratulations!", game.get_turn());
}
