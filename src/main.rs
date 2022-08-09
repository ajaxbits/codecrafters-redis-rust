use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

fn handle_connection(mut stream: TcpStream) {
    let mut data = [0; 128];
    while match stream.read(&mut data) {
        Ok(_) => {
            stream.write(b"+PONG").unwrap();
            true
        }
        Err(_) => {
            println!(
                "An error occured! Terminating connection with {}",
                stream.peer_addr().unwrap()
            );
            stream
                .shutdown(std::net::Shutdown::Both)
                .expect("could not shutdown connection");
            false
        }
    } {}
}

fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    // Uncomment this block to pass the first stage
    // let listener = TcpListener::bind("127.0.0.1:6379").unwrap();
    let listener = TcpListener::bind("127.0.0.1:6379").expect("could not bind to port");
    for stream_result in listener.incoming() {
        match stream_result {
            Ok(stream) => handle_connection(stream),
            Err(e) => println!("couldn't accept client: {:?}", e),
        }
    }
}
