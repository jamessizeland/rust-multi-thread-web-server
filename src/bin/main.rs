use std::io::prelude::*; // We bring into scope to get access to certain traits that let us read from and write to the stream
use std::net::{TcpListener, TcpStream};
use std::time::Duration;
use std::{fs, thread};
use webserver::ThreadPool;

/// Main
///
/// nonadministrators can listen only on ports higher than 1023
///
/// <https://doc.rust-lang.org/book/ch20-02-multithreaded.html>
///
/// Because we’re writing a basic server just for learning purposes, we won’t worry about handling these kinds of errors; instead, we use unwrap to stop the program if errors happen.
fn main() {
    println!("Hello, from my multithreaded web service");
    let listener = TcpListener::bind("localhost:7878").unwrap();
    let pool = ThreadPool::new(4);

    // A single stream represents an open connection between the client and the server
    for stream in listener.incoming() {
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_connection(stream);
        });
    }
}

/// Handle routing of requests
fn handle_connection(mut stream: TcpStream) {
    println!("Connection Established!!");

    // read http get request
    let mut buffer = [0; 1024]; // 1024 bytes in size
    stream.read(&mut buffer).unwrap();
    println!("Request {}", String::from_utf8_lossy(&buffer[..]));
    // The “lossy” part of the name indicates the behavior of this function when it sees an invalid UTF-8 sequence

    // respond to http get request
    let get = b"GET / HTTP/1.1\r\n";
    let sleep = b"GET /sleep HTTP/1.1\r\n";

    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK", "hello.html")
    } else if buffer.starts_with(sleep) {
        thread::sleep(Duration::from_secs(5));
        ("HTTP/1.1 200 OK", "hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "404.html")
    };

    let contents = fs::read_to_string(filename).unwrap();
    let response = format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        status_line,
        contents.len(),
        contents
    );

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap(); // wait and prevent the program from continuing until all the bytes are written to the connection
}
