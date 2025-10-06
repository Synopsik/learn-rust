use std::{
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};


fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    
    for stream in listener.incoming() {
        let stream = stream.unwrap(); // uses shadowing to unwrap Result<>
        
        println!("Connection established!"); // Broadcast to user
    }
}