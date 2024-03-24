// Importing dependencies for building the TCP server 
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

fn handle_client (mut stream: TcpStream) {
    // to read data from client
    let mut buffer = [0; 1024];
    // this reads and stores data from the buffer and stores it in buffer
    stream.read(&mut buffer).expect("failed to read from client");
    // converts the data in the buffer into an UTF-8 encoded string
    let request = String::from_utf8_lossy(&buffer[..]);
    println!("Recieved request: {} ", request );

    let response = "Hello, Client".as_bytes();
    stream.write(response).expect("Failed to write data");
}

fn main() {
    // setting up a listener variable
    let listener = TcpListener::bind("127.0.0.1:8080").expect("Failed to bind connection");
    println!("Server listening at localhost:8080");

    // allocating a new thread to each client as a response
    for stream in listener.incoming(){
        match stream{
            Ok(stream) => {
                std::thread::spawn(|| handle_client(stream));
            }
            Err(e) => {
                // stderr => standard error stream
                eprintln!("Failed to connect: {}", e);
            }
        }
    }
}