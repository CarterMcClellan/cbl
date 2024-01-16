use crate::token::{Token, TokenType};

pub type CblResult<T> = Result<T, Error>;

pub fn error(line: u32, message: &str) {
    report(line, "", message);
}

pub fn report(line: u32, where_: &str, message: &str) {
    eprintln!("[line {}] Error {}: {}", line, where_, message);
}

pub fn parser_error(token: &Token, message: &str) {
    if token.type_ == TokenType::Eof {
        report(token.line, " at end", message);
    } else {
        report(token.line, &format!(" at '{}'", token.lexeme), message);
    }
}

#[derive(Debug)]
pub enum Error {
    ParserError(String),
    RuntimeError(String),
}

impl Error {
    pub fn new(message: &str) -> Error {
        Error::ParserError(message.to_string())
    }

    pub fn parser_error(message: &str) -> Error {
        Error::ParserError(message.to_string())
    }

    pub fn runtime_error(message: &str) -> Error {
        Error::RuntimeError(message.to_string())
    }
}