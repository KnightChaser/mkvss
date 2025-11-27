// src/http/response.rs

use super::status_code::StatusCode;
use std::io::{Result as IoResult, Write};
use std::net::TcpStream;

#[derive(Debug)]
pub struct Response {
    pub status_code: StatusCode,
    pub body: Option<String>,
}

impl Response {
    pub fn new(status_code: StatusCode, body: Option<String>) -> Self {
        Response { status_code, body }
    }

    pub fn send(&self, stream: &mut TcpStream) -> IoResult<()> {
        let body = match &self.body {
            Some(body) => body,
            None => "",
        };

        // Format: "HTTP/1.1 <status_code> <reason_phrase>\r\n\r\n<body>"
        write!(
            stream,
            "HTTP/1.1 {} {}\r\n\r\n{}",
            self.status_code as u16,
            self.status_code.reason_phrase(),
            body
        )
    }
}
