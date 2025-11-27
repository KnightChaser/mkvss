// src/http/status_code.rs

use std::fmt::{Display, Formatter, Result};

#[derive(Copy, Clone, Debug)]
pub enum StatusCode {
    Ok = 200,
    Created = 201,
    BadRequest = 400,
    NotFound = 404,
    InternalServerError = 500,
}

impl StatusCode {
    pub fn reason_phrase(&self) -> &str {
        match self {
            StatusCode::Ok => "OK",
            StatusCode::Created => "Created",
            StatusCode::BadRequest => "Bad Request",
            StatusCode::NotFound => "Not Found",
            StatusCode::InternalServerError => "Internal Server Error",
        }
    }
}

/// An implementation to display the status code as its numeric value.
/// e.g., StatusCode::Ok -> "200"
impl Display for StatusCode {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{}", *self as u16)
    }
}
