// src/http/request.rs

use super::method::Method;
use std::io::{BufRead, BufReader, Read};
use std::net::TcpStream;
use std::str::FromStr;

#[derive(Debug)]
pub struct Request {
    pub method: Method,
    pub path: String,
    pub body: Option<String>, // TODO: Handle body properly
}

impl Request {
    pub fn parse(stream: &mut TcpStream) -> Option<Self> {
        let mut reader = BufReader::new(stream);

        // Read the request line
        let mut request_line = String::new();
        if reader.read_line(&mut request_line).is_err() || request_line.is_empty() {
            return None;
        }

        let parts: Vec<&str> = request_line.split_whitespace().collect();
        if parts.len() < 2 {
            // Invalid request line
            return None;
        }

        // Parse method
        let method = Method::from_str(parts[0]).ok()?;
        let path = parts[1].to_string();

        // Read remaining headers
        let mut content_length = 0;
        loop {
            let mut line = String::new();
            reader.read_line(&mut line).ok()?;

            if line == "\r\n" || line == "\n" {
                break; // End of headers
            }

            // TODO:
            // Find the content-length without a full header map yet
            if line.to_lowercase().starts_with("content-length:") {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if let Some(len_str) = parts.get(1) {
                    content_length = len_str.trim().parse::<usize>().unwrap_or(0);
                }
            }
        }

        // Read body if content-length is specified (>0)
        let mut body = None;
        if content_length > 0 {
            // Read exact bytes, not lines
            let mut buffer = vec![0; content_length];
            reader.read_exact(&mut buffer).ok()?;
            body = Some(String::from_utf8_lossy(&buffer).to_string());
        }

        Some(Request { method, path, body })
    }
}
