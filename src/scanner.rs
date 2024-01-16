use std::fmt::Display;

use crate::token::{Object, Token, TokenType};

pub struct Scanner {
    source: String,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: u32,
}

impl Scanner {
    pub fn new(source: &str) -> Scanner {
        Scanner {
            source: source.to_string(),
            tokens: vec![],
            start: 0,
            current: 0,
            line: 1,
        }
    }

    /// Scan its way through the source file then append one
    /// final EOF token
    pub fn scan_tokens(&mut self) -> Vec<Token> {
        while !self.is_at_end(self.current) {
            self.start = self.current;
            self.scan_token();
        }

        self.tokens.push(Token::new(
            TokenType::Eof,
            "".to_string(),
            Object::Nil,
            self.line,
        ));

        self.tokens.clone()
    }

    /// Check if the scanner has reached the end of the source file
    fn is_at_end(&self, current: usize) -> bool {
        current >= self.source.len()
    }

    /// Add a token to the list of tokens
    fn scan_token(&mut self) {
        let opt_c = self.advance();
        if let Some(c) = opt_c {
            match c {
                '(' => self.add_token(TokenType::LeftParen),
                ')' => self.add_token(TokenType::RightParen),
                '{' => self.add_token(TokenType::LeftBrace),
                '}' => self.add_token(TokenType::RightBrace),
                ',' => self.add_token(TokenType::Comma),
                '.' => self.add_token(TokenType::Dot),
                '-' => self.add_token(TokenType::Minus),
                '+' => self.add_token(TokenType::Plus),
                ';' => self.add_token(TokenType::Semicolon),
                '*' => self.add_token(TokenType::Star),
                '!' => {
                    let type_ = if self.match_char('=') {
                        TokenType::BangEqual
                    } else {
                        TokenType::Bang
                    };
                    self.add_token(type_);
                }
                '=' => {
                    let type_ = if self.match_char('=') {
                        TokenType::EqualEqual
                    } else {
                        TokenType::Equal
                    };
                    self.add_token(type_);
                }
                '<' => {
                    let type_ = if self.match_char('=') {
                        TokenType::LessEqual
                    } else {
                        TokenType::Less
                    };
                    self.add_token(type_);
                }
                '>' => {
                    let type_ = if self.match_char('=') {
                        TokenType::GreaterEqual
                    } else {
                        TokenType::Greater
                    };
                    self.add_token(type_);
                }
                '/' => {
                    // this represents a comment and we should ignore
                    // until we see a newline character
                    if self.match_char('/') {
                        while self.peek() != '\n' && !self.is_at_end(self.current) {
                            self.advance();
                        }
                    } else {
                        self.add_token(TokenType::Slash);
                    }
                }
                // ignore whitespace
                ' ' | '\r' | '\t' | '\n' => {}
                '"' => self.string(),
                _ => {
                    if self.is_digit(c) {
                        self.number();
                    } else if self.is_alpha(c) {
                        self.identifier();
                    } else {
                        println!("Unexpected character: {}", c);
                        println!("char value {}", c as u32)
                    }
                }
            }
        }
    }

    /// Advance the scanner one character
    fn advance(&mut self) -> Option<char> {
        self.current += 1;
        self.source.chars().nth(self.current - 1)
    }

    fn add_token(&mut self, type_: TokenType) {
        self.add_token_literal(type_, Object::Nil)
    }

    fn add_token_literal(&mut self, type_: TokenType, literal: Object) {
        let text = self.source[self.start..self.current].to_string();
        self.tokens
            .push(Token::new(type_, text, literal, self.line));
    }

    /// Check if the current character matches the expected character
    fn match_char(&mut self, expected: char) -> bool {
        if self.is_at_end(self.current) {
            return false;
        }

        let opt_c = self.source.chars().nth(self.current);
        if let Some(c) = opt_c {
            if c != expected {
                return false;
            }
        }

        self.current += 1;
        true
    }

    /// Look at the next character without advancing the scanner
    fn peek(&self) -> char {
        if self.is_at_end(self.current) {
            return '\0';
        }

        self.source.chars().nth(self.current).unwrap()
    }

    /// Look at the character after the next character without advancing the scanner
    fn peek_next(&self) -> char {
        if self.current + 1 >= self.source.len() {
            return '\0';
        }

        self.source.chars().nth(self.current + 1).unwrap()
    }

    /// Store all of the characters between '"' and '"'
    fn string(&mut self) {
        while self.peek() != '"' && !self.is_at_end(self.current) {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.advance();
        }

        if self.is_at_end(self.current) {
            println!("Unterminated string");
            return;
        }

        // consume the closing "
        self.advance();

        let value = self.source[self.start + 1..self.current - 1].to_string();
        self.add_token_literal(TokenType::String, Object::String(value));
    }

    fn is_digit(&self, c: char) -> bool {
        '0' <= c && c <= '9'
    }

    fn is_alpha(&self, c: char) -> bool {
        ('a' <= c && c <= 'z') || ('A' <= c && c <= 'Z') || c == '_'
    }

    fn is_alpha_numeric(&self, c: char) -> bool {
        self.is_alpha(c) || self.is_digit(c)
    }

    /// Store all of the characters between '0' and '9'
    fn number(&mut self) {
        while self.is_digit(self.peek()) {
            self.advance();
        }

        // look for a fractional part
        if self.peek() == '.' && self.is_digit(self.peek_next()) {
            // consume the '.'
            self.advance();

            while self.is_digit(self.peek()) {
                self.advance();
            }
        }

        let value = self.source[self.start..self.current]
            .parse::<f64>()
            .unwrap();
        self.add_token_literal(TokenType::Number, Object::Number(value));
    }

    /// Store all of the characters between 'a' and 'z' or 'A' and 'Z'
    fn identifier(&mut self) {
        while self.is_alpha(self.peek()) || self.is_digit(self.peek()) {
            self.advance();
        }

        let text = self.source[self.start..self.current].to_string();
        let type_ = match text.as_str() {
            "and" => TokenType::And,
            "class" => TokenType::Class,
            "else" => TokenType::Else,
            "false" => TokenType::False,
            "for" => TokenType::For,
            "fun" => TokenType::Fun,
            "if" => TokenType::If,
            "nil" => TokenType::Nil,
            "or" => TokenType::Or,
            "print" => TokenType::Print,
            "return" => TokenType::Return,
            "super" => TokenType::Super,
            "this" => TokenType::This,
            "true" => TokenType::True,
            "var" => TokenType::Var,
            "while" => TokenType::While,
            _ => TokenType::Identifier,
        };

        self.add_token(type_);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scanner() {
        let mut scanner = Scanner::new("-123");
        let tokens = scanner.scan_tokens();
    
        let expected = vec![
            Token::new(TokenType::Minus, String::from("-"), Object::Nil, 1),
            Token::new(TokenType::Number, String::from("123"), Object::Number(123.0), 1),
            Token::new(TokenType::Eof, String::from(""), Object::Nil, 1),
        ];
    
        assert_eq!(tokens, expected);
    }
    
}