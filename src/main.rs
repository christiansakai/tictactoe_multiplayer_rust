extern crate tictactoe;

extern crate serde_json;

use std::env;
use std::io::{self, Write};

use tictactoe::game::{TicTacToe, MultiplayerStatus};
use tictactoe::network::{server, client};
use tictactoe::util::{self};

const ADDRESS: &'static str = "127.0.0.1:6000";
const MESSAGE_SIZE: usize = 32;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 && args[1] == "server" {
        play_lan_as_server();
    } else if args.len() > 1 && args[1] == "client" {
        play_lan_as_client();
    } else {
        play_hotseat();
    }
}

fn play_lan_as_client() {
    util::clear_terminal();
    println!("Initialize as Client");

    let client = client::Client::connect(ADDRESS, MESSAGE_SIZE);

    loop {
        let message = client.receive_message().unwrap();
        let multiplayer_status: MultiplayerStatus = serde_json::from_str(&message).unwrap();

        if let MultiplayerStatus::ServerAskInputFromClient(game) = multiplayer_status {
            println!();
            println!("{}", game.get_board());
            println!();
            println!("Turn:  Player {}", game.get_turn());
            println!();

            let input = util::get_user_input();
            let multiplayer_status = MultiplayerStatus::ClientGiveInputToServer(input);

            let message = serde_json::to_string(&multiplayer_status).unwrap();
            client.send_message(&message); 
        } else if let MultiplayerStatus::GameOver { game, winner } = multiplayer_status {
            let game_board = game.get_board();
            let winning_player = game.get_turn().to_string();

            util::display_outro(game_board, winning_player);
            break;
        }
    }
}

fn play_lan_as_server() {
    util::clear_terminal();
    println!("Initialize as Server");

    let mut game = TicTacToe::new();
    let mut server = server::Server::listen(ADDRESS, MESSAGE_SIZE);

    while server.clients_count() < 2 {
        server.accept_client(game.get_player_from_bench());
    }

    loop {
        let player = game.get_turn();
        let multiplayer_status = MultiplayerStatus::ServerAskInputFromClient(game.clone());
        let message = serde_json::to_string(&multiplayer_status).unwrap();
        server.send_message(player, &message).unwrap();

//         let message = server.receive_message().unwrap();

//         match game.fill(input.row, input.col) {
//             Ok(_) => {
//                 if game.check_game_over() {
//                     break;
//                 } else {
//                     let _ = game.next_turn();
//                     continue;
//                 }
//             }
//             Err(msg) => {
//                 println!("Invalid move, {}", msg);
//                 continue;
//             }
//         }
    }

    let winner = game.get_turn().to_string();

//     server.send_message(winner, game_board, "You win!");
//     server.send_message(loser, game_board, "You lose!");
}

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
