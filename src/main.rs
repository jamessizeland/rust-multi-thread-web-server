use std::fs;
use std::io::prelude::*; // We bring into scope to get access to certain traits that let us read from and write to the stream
use std::net::{TcpListener, TcpStream};

/// Simple TCP Web Service
///
/// nonadministrators can listen only on ports higher than 1023
///
/// <https://doc.rust-lang.org/book/ch20-01-single-threaded.html>
///
/// Because we’re writing a basic server just for learning purposes, we won’t worry about handling these kinds of errors; instead, we use unwrap to stop the program if errors happen.
fn main() {
    println!("Hello, from my web service");
    let listener = TcpListener::bind("localhost:7878").unwrap();
    // A single stream represents an open connection between the client and the server
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    println!("Connection Established!!");

    // read http get request
    let mut buffer = [0; 1024]; // 1024 bytes in size
    stream.read(&mut buffer).unwrap();
    println!("Request {}", String::from_utf8_lossy(&buffer[..]));
    // The “lossy” part of the name indicates the behavior of this function when it sees an invalid UTF-8 sequence

    // respond to http get request
    let contents = fs::read_to_string("hello.html").unwrap();
    let response = format!(
        "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
        contents.len(),
        contents
    );

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap(); // wait and prevent the program from continuing until all the bytes are written to the connection
}
