// src/http/method.rs

use std::str::FromStr;

#[derive(Debug)]
pub enum Method {
    GET,
    POST,
    PUT,
    DELETE,
}

/// A simple implementation to parse HTTP methods from strings.
/// e.g. "GET" -> Method::GET
impl FromStr for Method {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "GET" => Ok(Method::GET),
            "POST" => Ok(Method::POST),
            "PUT" => Ok(Method::PUT),
            "DELETE" => Ok(Method::DELETE),
            _ => Err(()),
        }
    }
}
