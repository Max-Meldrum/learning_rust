use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;


pub fn listen() {
    let listener = TcpListener::bind("127.0.0.1:5000").unwrap();
    
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        println!("Connection established!");
        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 512];
    stream.read(&mut buffer).unwrap();
    println!("Request: {}", String::from_utf8_lossy(&buffer[..]));
}
