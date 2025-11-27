// src/main.rs

mod http;

use http::request::Request;
use http::response::Response;
use http::status_code::StatusCode;

use std::net::{TcpListener, TcpStream};
use std::thread;

fn handle_client(mut stream: TcpStream) {
    let response = match Request::parse(&mut stream) {
        Some(req) => {
            println!("Received: {:?} {}", req.method, req.path);

            // Logic Placeholder: "If path is /test, say OK"
            if req.path == "/test" {
                Response::new(StatusCode::Ok, Some("You found the test page!".to_string()))
            } else {
                Response::new(StatusCode::NotFound, Some("Page not found".to_string()))
            }
        }
        None => Response::new(StatusCode::BadRequest, None),
    };

    // The response object handles the low-level writing
    if let Err(e) = response.send(&mut stream) {
        println!("Failed to send response: {}", e);
    }
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    println!("Server running on 127.0.0.1:8080");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(move || {
                    handle_client(stream);
                });
            }
            Err(e) => println!("Connection failed: {}", e),
        }
    }
}
