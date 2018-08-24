use std::net::{TcpListener, TcpStream, Shutdown};
use std::sync::mpsc::{self, Receiver, Sender};
use std::io::{Read, Write, ErrorKind};
use std::thread;

use util;

const ADDRESS: &'static str = "127.0.0.1:6000";
const MESSAGE_SIZE: usize = 32;

pub fn listen() {
    // Main Thread: Start listening as the Server
    let server = TcpListener::bind(ADDRESS)
        .expect("Failed to bind");

    server
        .set_nonblocking(true) // TODO: Why non-blocking?
        .expect("Failed to initialize non-blocking");

    // Main Thread: Create channel for Main <-> Handler Thread communication
    let (tx, rx): (Sender<String>, Receiver<String>) = mpsc::channel();

    let mut clients: Vec<TcpStream> = vec![];

    loop {
        if let Ok((mut socket, address)) = server.accept() {
            if clients.len() == 2 {
                println!("Max # of clients achieved, close connection with Client {}", address);
                socket.shutdown(Shutdown::Both)
                    .expect("Shutdown attempt failed");
                continue;
            }

            println!("Client {} connected", address);

            let client = socket.try_clone()
                .expect(&format!("Failed to clone client {}", address));
            clients.push(client);
            println!("Connected clients: {}", clients.len());

            let tx = tx.clone();

            // Main Thread: Spawn a Handler Thread per connected Client
            thread::spawn(move | | loop {
                let mut buffer = vec![0; MESSAGE_SIZE];

                // Handler Thread: Receives message from the Client
                match socket.read_exact(&mut buffer) {
                    Ok(_) => {
                        let message_bytes: Vec<_> = buffer.into_iter()
                            .take_while(|&x| x != 0)
                            .collect();

                        let message = String::from_utf8(message_bytes)
                            .expect("Invalid UTF-8 message");

                        println!("Received from client {}, message \"{}\"", address, message);

                        // Handler Thread: Sends received message to Main Thread
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

        // Main Thread: Receive message from Handler Thread
        if let Ok(message) = rx.try_recv() {
            clients = clients
                .into_iter()
                .filter_map(|mut client| {
                    let mut buffer = message.clone().into_bytes();
                    buffer.resize(MESSAGE_SIZE, 0);

                    // Main Thread: Echo the message back to the Client
                    // Only filter Client which are still connected
                    client
                        .write_all(&buffer)
                        .map(|_| client)
                        .ok()
                })
                .collect();

            util::sleep(100);
        }
    }

}
