use std::net::{TcpListener, TcpStream, Shutdown};
use std::sync::mpsc::{self, Receiver, Sender, SendError, RecvError};
use std::io::{Read, Write, ErrorKind};
use std::thread;

use super::super::game::{Player};

use util;

const ADDRESS: &'static str = "127.0.0.1:6000";
const MESSAGE_SIZE: usize = 32;

struct Channel {
    sender: Sender<String>,
    receiver: Receiver<String>,
}

struct ClientHandler {
    address: String,
    name: Player,
    channel: Channel,
}

pub struct Server {
    server: TcpListener,
    channel: Channel,
    clients: Vec<ClientHandler>,
}

impl Server {
    pub fn listen(address: &str, messages: usize) -> Server {
        // Main Thread: Start listening as the Server
        let server = TcpListener::bind(ADDRESS)
            .expect("Failed to bind");

        server
            .set_nonblocking(true)
            .expect("Failed to initialize non-blocking");

        let clients: Vec<ClientHandler> = vec![];

        // Main Thread: Create channel for Handler Thread -> Main Thread communication
        let (sender, receiver): (Sender<String>, Receiver<String>) = mpsc::channel();
        let channel = Channel { sender, receiver };

        Server { server, channel, clients }
    }

    pub fn clients_count(&self) -> usize {
        self.clients.len()
    }

    pub fn accept_client(&mut self, player_to_be_assigned: Player) {
        if let Ok((mut socket, address)) = self.server.accept() {

            let mut socket = socket.try_clone()
                .expect(&format!("Failed to clone client {}", address));

            // Main Thread: Create channel for Main Thread -> Handler Thread communication
            let (sender, receiver): (Sender<String>, Receiver<String>) = mpsc::channel();

            let client_handler = ClientHandler {
                address: address.to_string(),
                name: player_to_be_assigned,
                channel: Channel { sender, receiver },
            };

            self.clients.push(client_handler);

            println!("Client {} connected", address);
            println!("Connected clients: {}", self.clients.len());

            // Main Thread: Clone Main Thread's sender to be assigned to a Handler Thread
            let server_channel_sender = self.channel.sender.clone();

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
                        server_channel_sender.send(message)
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

    pub fn send_message(&self, client: Player, message: &str) -> Result<(), SendError<String>> {
    //     // self.main_sender.send(message.to_string())
        Ok(())
    }

    pub fn receive_message(&self) -> Result<String, RecvError> {
        self.channel.receiver.recv()
    }
}

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
