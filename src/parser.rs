use crate::{
    ast::Expr,
    token::{Object, Token, TokenType}, error::{Error, CblResult},
};

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, current: 0 }
    }

    /// Parse the tokens into an AST.
    pub fn parse(&mut self) -> CblResult<Expr> {
        self.expression()
    }

    fn expression(&mut self) -> CblResult<Expr> {
        self.equality()
    }

    fn equality(&mut self) -> CblResult<Expr> {
        let mut expr = match self.comparison() {
            Ok(expr) => expr,
            Err(e) => return Err(e) 
        };

        while self.match_token(vec![TokenType::BangEqual, TokenType::EqualEqual]) {
            let operator = self.previous();
            let right = match self.comparison() {
                Ok(expr) => expr,
                Err(e) => return Err(e),
            };
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    fn match_token(&mut self, types: Vec<TokenType>) -> bool {
        for token in types {
            if self.check(token) {
                self.advance();
                return true;
            }
        }

        false
    }

    fn check(&self, type_: TokenType) -> bool {
        if self.is_at_end() {
            return false;
        }

        self.peek().type_ == type_
    }

    fn advance(&mut self) -> Token {
        if !self.is_at_end() {
            self.current += 1;
        }

        self.previous()
    }

    fn is_at_end(&self) -> bool {
        self.peek().type_ == TokenType::Eof
    }

    fn peek(&self) -> Token {
        self.tokens[self.current].clone()
    }

    fn previous(&self) -> Token {
        self.tokens[self.current - 1].clone()
    }

    fn comparison(&mut self) -> CblResult<Expr> {
        let mut expr = match self.term() {
            Ok(expr) => expr,
            Err(e) => return Err(e),
        };

        while self.match_token(vec![
            TokenType::Greater,
            TokenType::GreaterEqual,
            TokenType::Less,
            TokenType::LessEqual,
        ]) {
            let operator = self.previous();
            let right = match self.term() {
                Ok(expr) => expr,
                Err(e) => return Err(e),
            };
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    fn term(&mut self) -> CblResult<Expr> {
        let mut expr = match self.factor() {
            Ok(expr) => expr,
            Err(e) => return Err(e),
        };

        while self.match_token(vec![TokenType::Minus, TokenType::Plus]) {
            let operator = self.previous();
            let right = match self.factor() {
                Ok(expr) => expr,
                Err(e) => return Err(e),
            };
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    fn factor(&mut self) -> CblResult<Expr> {
        let mut expr = match self.unary() {
            Ok(expr) => expr,
            Err(e) => return Err(e),
        };


        while self.match_token(vec![TokenType::Slash, TokenType::Star]) {
            let operator = self.previous();
            let right = match self.unary() {
                Ok(expr) => expr,
                Err(e) => return Err(e),
            };
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    fn unary(&mut self) -> CblResult<Expr> {
        if self.match_token(vec![TokenType::Bang, TokenType::Minus]) {
            let operator = self.previous();
            let right = match self.unary() {
                Ok(expr) => expr,
                Err(e) => return Err(e),
            };

            return Ok(Expr::Unary {
                operator,
                right: Box::new(right),
            });
        }

        self.primary()
    }

    fn primary(&mut self) -> CblResult<Expr> {
        if self.match_token(vec![TokenType::False]) {
            return Ok(Expr::Literal {
                value: Object::Bool(false),
            });
        }

        if self.match_token(vec![TokenType::True]) {
            return Ok(Expr::Literal {
                value: Object::Bool(true),
            });
        }

        if self.match_token(vec![TokenType::Nil]) {
            return Ok(Expr::Literal { value: Object::Nil });
        }

        if self.match_token(vec![TokenType::Number, TokenType::String]) {
            return Ok(Expr::Literal {
                value: self.previous().literal,
            });
        }

        if self.match_token(vec![TokenType::LeftParen]) {
            let expr = match self.expression() {
                Ok(expr) => expr,
                Err(e) => return Err(e),
            };
            match self.consume(TokenType::RightParen, "Expect ')' after expression.") {
                Ok(_) => {}
                Err(e) => return Err(e),
            };
            return Ok(Expr::Grouping {
                expression: Box::new(expr),
            });
        }

        Err(Error::parser_error("Expect expression."))
    }

    fn consume(&mut self, type_: TokenType, message: &str) -> CblResult<Token> {
        if self.check(type_) {
            return Ok(self.advance());
        }

        Err(Error::parser_error(message))
    }

    /// Discard tokens until we reach a statement boundary.
    /// This is used to recover from parse errors.
    fn synchronize(&mut self) {
        self.advance();

        while !self.is_at_end() {
            if self.previous().type_ == TokenType::Semicolon {
                return;
            }

            match self.peek().type_ {
                TokenType::Class
                | TokenType::Fun
                | TokenType::Var
                | TokenType::For
                | TokenType::If
                | TokenType::While
                | TokenType::Print
                | TokenType::Return => return,
                _ => {}
            }

            self.advance();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::scanner::Scanner;
    use crate::ast::AstPrinter;

    #[test]
    fn test_parser() {
        let mut scanner = Scanner::new("-123 * 45.67");
        let tokens = scanner.scan_tokens();

        let mut parser = Parser::new(tokens);
        let expression = parser.parse().expect("Could not parse sample code.");
        let printer = AstPrinter;

        assert_eq!(printer.print(expression).unwrap(), "(* (- 123) 45.67)");
    }
}