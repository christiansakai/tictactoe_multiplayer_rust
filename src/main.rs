extern crate tictactoe;

use std::io;
// use std::io::{self, Read, Write, ErrorKind};

use tictactoe::{TicTacToe};

fn main() {
    let mut game = TicTacToe::new();

    println!("----------------------");
    println!("Welcome to Tic Tac Toe");
    println!("----------------------");
    println!();

    loop {
        println!("=====================================");
        println!();
        println!("{}", game.get_board());
        println!();
        println!("Turn:  Player {}", game.get_turn());
        println!();

        // TODO
        // How to write input on this line
        println!("Input your row and col separated by space: ");

        let mut buffer = String::new();
        io::stdin()
            .read_line(&mut buffer)
            .expect("reading from stdin failed");

        let input_str = buffer
            .trim()
            .to_string();

        let input_vec: Vec<i32> = input_str
            .split_whitespace()
            // TODO
            // How to validate user input
            // so that user only inputs integer
            // between 0 and 2
            .map(|el| el.parse().unwrap())
            .collect();

        let row = input_vec[0];
        let col = input_vec[1];
        let turn = game.get_turn();

        println!();
        println!(
            "Player {} chose row: {}, col: {}", 
            turn,
            row,
            col,
        );
        println!();

        // TODO
        // Need to handle error here
        game.fill(row, col).unwrap();
        game.next_turn();

        if game.check_game_over() {
            break;
        }
    }

    println!("----------------------");
    println!("      Game Over       ");
    println!("----------------------");
    println!();
    println!("{}", game.get_board());
    println!();
    println!("Player {} wins! Congratulations!", game.get_turn());
}

