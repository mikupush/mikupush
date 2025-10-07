use std::error::Error;
use std::fmt::Display;

#[derive(Debug)]
pub struct ParseError {
    message: String,
}

impl Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl Error for ParseError {}

impl ParseError {
    pub fn new(message: &str) -> Self {
        Self { message: message.to_string() }
    }
}