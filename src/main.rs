use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;

mod resp;

fn handle_client(mut stream: TcpStream) {
    let mut data = [0; 128];
    while match stream.read(&mut data) {
        Ok(_) => {
            stream.write_all(b"+PONG\r\n").unwrap();
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
    let listener = TcpListener::bind("127.0.0.1:6379").expect("could not bind to port");
    for stream_result in listener.incoming() {
        match stream_result {
            Ok(stream) => {
                thread::spawn(move || handle_client(stream));
            }
            Err(e) => println!("couldn't accept client: {:?}", e),
        }
    }
}
