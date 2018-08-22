use std::net::{TcpStream};
use std::sync::mpsc::{self, Receiver, Sender, TryRecvError};
use std::io::{self, Read, Write, ErrorKind};
use std::thread;

use util;

const ADDRESS: &'static str = "127.0.0.1:6000";
const MESSAGE_SIZE: usize = 32;

pub fn connect() {
    // Start client
    let mut client = TcpStream::connect(ADDRESS)
        .expect(&format!("Failed to connect to server {}", ADDRESS));

    client
        .set_nonblocking(true)
        .expect("Failed to initiate non-blocking");

    // Create channel for thread communication
    let (tx, rx): (Sender<String>, Receiver<String>) = mpsc::channel();

    // Client's handler thread
    thread::spawn(move | | loop {
        let mut buffer = vec![0; MESSAGE_SIZE];

        // Receives message from the server
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

        // Receives user input from main thread
        match rx.try_recv() {
            Ok(message) => {
                let mut buffer = message.clone().into_bytes();
                buffer.resize(MESSAGE_SIZE, 0);

                client.write_all(&buffer)
                    .expect("Writing to socket failed");

                println!("Message sent to server \"{:?}\"", message);
            },
            Err(TryRecvError::Empty) => (),
            Err(TryRecvError::Disconnected) => break,
        }

        util::sleep(100);
    });

    // Client's main thread
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

        tx.send(message)
            .expect("Failed to send message to rx");
    }

    println!("Bye!");
}
