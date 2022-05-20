use std::net::TcpListener;

fn main() {
    println!("Hello, from my web service");

    // https://doc.rust-lang.org/book/ch20-01-single-threaded.html
    let listener = TcpListener::bind("localhost:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        println!("Connection Established!!");
    }
}
