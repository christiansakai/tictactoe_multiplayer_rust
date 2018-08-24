extern crate tictactoe;

use std::env;
use std::io::{self, Write};

use tictactoe::game::{TicTacToe};
use tictactoe::network::{server, client};
use tictactoe::util::{self, Input};

const ADDRESS: &'static str = "127.0.0.1:6000";
const MESSAGE_SIZE: usize = 32;

fn main() {
    util::clear_terminal();

    let args: Vec<String> = env::args().collect();

    if args.len() > 1 && args[1] == "server" {
        println!("Initialize as Server");
        // println!("Not implemented yet");
        server::listen();
    } else if args.len() > 1 && args[1] == "client" {
        println!("Initialize as Client");
        // println!("Not implemented yet");
        play_lan();
        // client::connect();
    } else {
        play_hotseat();
    }

}

fn play_lan() {
    util::clear_terminal();

    // Client
    let client = client::Client::connect(ADDRESS, MESSAGE_SIZE);

    loop {
        client.send_message("hellow");

        let message = client.receive_message().unwrap();
        println!("{}", message);

    //  msg.render_board();
    //  let (col, row) = get_col_row_from_user() 
    //  client.send_to_server(col, row);
    }
}

// Pseudo code
// fn play_lan() {
//     util::clear_terminal();

//     // Server
//     let mut game = TicTacToe::new();
//     let server = Server::listen(ADDRESS, MESSAGE_SIZE);

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
//
//     // Client
//     let client = Client::connect(ADDRESS);
//
//     loop {
//      let msg = client.wait_for_receive_message_from_server();
//      msg.render_board();
//      let (col, row) = get_col_row_from_user() 
//      client.send_to_server(col, row);
//     }
// }

fn play_hotseat() {
    util::clear_terminal();

    let start_countdown: usize = 3;
    util::display_intro(start_countdown);

    let mut game = TicTacToe::new();

    loop {
        util::clear_terminal();

        println!();
        println!("{}", game.get_board());
        println!();
        println!("Turn:  Player {}", game.get_turn());
        println!();

        // Ask user input
        let input = util::get_user_input();
        let turn = game.get_turn();

        println!();
        println!(
            "Player {} chose row: {}, col: {}", 
            turn,
            input.row,
            input.col,
        );
        println!();

        match game.fill(input.row, input.col) {
            Ok(_) => {
                if game.check_game_over() {
                    break;
                } else {
                    let _ = game.next_turn();
                    continue;
                }
            }
            Err(msg) => {
                println!("Invalid move, {}", msg);
                continue;
            }
        }
    }

    util::clear_terminal();

    let game_board = game.get_board();
    let winning_player = game.get_turn().to_string();
    util::display_outro(game_board, winning_player);
}
