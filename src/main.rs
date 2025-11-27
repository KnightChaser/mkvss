mod http;

use http::request::Request;
use std::io::Write;
use std::net::{TcpListener, TcpStream};
use std::thread;

fn handle_client(mut stream: TcpStream) {
    // Parse the request using handcrafted module
    match Request::parse(&mut stream) {
        Some(req) => {
            println!("Received: {:?} on {}", req.method, req.path);
            if let Some(body) = req.body {
                println!("Body payload: {}", body);
            }

            // Send a dummy HTTP 200 response
            let response = "HTTP/1.1 200 OK\r\n\r\nHello from Rust!";
            stream.write_all(response.as_bytes()).unwrap();
        }
        None => {
            println!("Failed to parse request");
        }
    }
}

fn main() {
    let address: &str = "127.0.0.1:8080";
    let listener = TcpListener::bind(address).unwrap();
    println!("Listening on {}", address);

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(|| handle_client(stream));
            }
            Err(e) => {
                eprintln!("Connection failed: {}", e);
            }
        }
    }
}
