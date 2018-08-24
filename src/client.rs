use std::net::{TcpStream};
use std::sync::mpsc::{self, Receiver, Sender, TryRecvError};
use std::io::{self, Read, Write, ErrorKind};
use std::thread;

use util;

const ADDRESS: &'static str = "127.0.0.1:6000";
const MESSAGE_SIZE: usize = 32;

pub fn connect() {
    // Main Thread: Start as the Client connecting to the Server
    let mut client = TcpStream::connect(ADDRESS)
        .expect(&format!("Failed to connect to server {}", ADDRESS));

    client
        .set_nonblocking(true) // TODO: Why non-blocking?
        .expect("Failed to initiate non-blocking");

    // Main Thread: Create channel for Main <-> Handler Thread communication
    let (tx, rx): (Sender<String>, Receiver<String>) = mpsc::channel();

    // Main Thread: Spawn a Handler Thread for communicating with the Server
    thread::spawn(move | | loop {
        let mut buffer = vec![0; MESSAGE_SIZE];

        // Handler Thread: Receive User's input from Main Thread
        match rx.try_recv() {
            Ok(message) => {
                let mut buffer = message.clone().into_bytes();
                buffer.resize(MESSAGE_SIZE, 0);

                // Handler Thread: Send message to the Server
                client.write_all(&buffer)
                    .expect("Writing to socket failed");

                println!("Message sent to server \"{:?}\"", message);
            },
            Err(TryRecvError::Empty) => (),
            Err(TryRecvError::Disconnected) => break,
        }

        // Handler Thread: Receive message from the Server
        match client.read_exact(&mut buffer) {
            Ok(_) => {
                let message_bytes: Vec<_> = buffer
                    .into_iter()
                    .take_while(|&x| x != 0)
                    .collect();

                println!("Message (in bytes) received {:?}", message_bytes);
            },
            Err(ref err) if err.kind() == ErrorKind::WouldBlock => (),
            Err(_) => {
                println!("Connection with the server was severed");
                break;
            }
        }

        util::sleep(100);
    });

    // Main Thread: Get text input from the User's console
    println!("Write a message:");

    loop {
        let mut buffer = String::new();

        io::stdin()
            .read_line(&mut buffer)
            .expect("Failed to read from stdin");

        let message = buffer.trim().to_string();

        if message == ":quit" {
            break;
        } 

        // Main Thread: Send User's input to Handler Thread
        tx.send(message)
            .expect("Failed to send message to rx");
    }

    println!("Bye!");
}
