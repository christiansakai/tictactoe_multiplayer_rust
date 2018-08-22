use std::net::{TcpListener, TcpStream};
use std::sync::mpsc::{self, Receiver, Sender};
use std::io::{Read, Write, ErrorKind};
use std::thread;

use util;

const ADDRESS: &'static str = "127.0.0.1:6000";
const MESSAGE_SIZE: usize = 32;

pub fn listen() {
    // Start server
    let server = TcpListener::bind(ADDRESS)
        .expect("Failed to bind");

    server
        .set_nonblocking(true)
        .expect("Failed to initialize non-blocking");

    // Create channel for thread communication
    let (tx, rx): (Sender<String>, Receiver<String>) = mpsc::channel();

    let mut clients: Vec<TcpStream> = vec![];

    while clients.len() <= 2 {
        if let Ok((mut socket, address)) = server.accept() {
            println!("Client {} connected", address);

            let client = socket.try_clone()
                .expect(&format!("Failed to clone client {}", address));
            clients.push(client);

            let tx = tx.clone();

            // Server's handler threads
            // one thread handles one connected client
            thread::spawn(move | | loop {
                let mut buffer = vec![0; MESSAGE_SIZE];

                // Receives message from a client
                match socket.read_exact(&mut buffer) {
                    Ok(_) => {
                        let message_bytes: Vec<_> = buffer.into_iter()
                            .take_while(|&x| x != 0)
                            .collect();

                        let message = String::from_utf8(message_bytes)
                            .expect("Invalid UTF-8 message");

                        println!("Received from client {}, message \"{}\"", address, message);

                        // Sends received message to main thread
                        tx.send(message)
                            .expect("Failed to send message to rx");
                    },
                    Err(ref err) if err.kind() == ErrorKind::WouldBlock => (),
                    Err(_) => {
                        println!("Closing connection with client {}", address);
                        break;
                    },
                }

                util::sleep(100);
            });

        }
    }

    // Server's main thread
    if let Ok(message) = rx.try_recv() {
        clients = clients
            .into_iter()
            .filter_map(|mut client| {
                let mut buffer = message.clone().into_bytes();
                buffer.resize(MESSAGE_SIZE, 0);

                // TODO
                // What does this do?
                client
                    .write_all(&buffer)
                    .map(|_| client)
                    .ok()
            })
            .collect();

        util::sleep(100);
    }
}
