use crate::error::{CblResult, Error};
use crate::token::{
    Object,
    Token, TokenType,
};
use crate::ast::{
    Visitor,
    Expr,
};

pub struct Interpreter {}

impl Visitor<Object> for Interpreter {

    fn visit_binary_expr(&self, left: &Expr, operator: &Token, right: &Expr) -> CblResult<Object> {
        let l = self.evaluate(left)?;
        let r = self.evaluate(right)?;
        
        // this is so much better than it looks in java because of match 
        match operator.type_ {
            // Numeric Operations
            TokenType::Minus => match (l, r) {
                (Object::Number(l), Object::Number(r)) => Ok(Object::Number(l - r)),
                _ => Err(Error::runtime_error(&format!("Expected numbers for Minus operation: {:?}", operator.type_))),
            },
            TokenType::Slash => match (l, r) {
                (Object::Number(l), Object::Number(r)) => Ok(Object::Number(l / r)),
                _ => Err(Error::runtime_error(&format!("Expected numbers for Slash operation: {:?}", operator.type_))),
            },
            TokenType::Star => match (l, r) {
                (Object::Number(l), Object::Number(r)) => Ok(Object::Number(l * r)),
                _ => Err(Error::runtime_error(&format!("Expected numbers for Star operation: {:?}", operator.type_))),
            },
            TokenType::Plus => match (l, r) {
                (Object::Number(l), Object::Number(r)) => Ok(Object::Number(l + r)),
                (Object::String(l), Object::String(r)) => Ok(Object::String(l + &r)),
                _ => Err(Error::runtime_error(&format!("Expected numbers or strings for Plus operation: {:?}", operator.type_))),
            },
            
            // Boolean Operations
            TokenType::Greater => match (l, r) {
                (Object::Number(l), Object::Number(r)) => Ok(Object::Bool(l > r)),
                _ => Err(Error::runtime_error(&format!("Expected numbers for Greater operation: {:?}", operator.type_))),
            },
            TokenType::GreaterEqual => match (l, r) {
                (Object::Number(l), Object::Number(r)) => Ok(Object::Bool(l >= r)),
                _ => Err(Error::runtime_error(&format!("Expected numbers for GreaterEqual operation: {:?}", operator.type_))),
            },
            TokenType::Less => match (l, r) {
                (Object::Number(l), Object::Number(r)) => Ok(Object::Bool(l < r)),
                _ => Err(Error::runtime_error(&format!("Expected numbers for Less operation: {:?}", operator.type_))),
            },
            TokenType::LessEqual => match (l, r) {
                (Object::Number(l), Object::Number(r)) => Ok(Object::Bool(l <= r)),
                _ => Err(Error::runtime_error(&format!("Expected numbers for Less operation: {:?}", operator.type_))),
            },
            TokenType::BangEqual => Ok(Object::Bool(!self.is_equal(&l, &r))),
            TokenType::EqualEqual => Ok(Object::Bool(self.is_equal(&l, &r))),
            _ => Err(Error::runtime_error(&format!("Unexpected token type: {:?}", operator.type_))),
        }
    }

    fn visit_grouping_expr(&self, expression: &Expr) -> CblResult<Object> {
        self.evaluate(expression)
    }

    fn visit_literal_expr(&self, value: &Object) -> CblResult<Object> {
        Ok(value.clone())
    }

    fn visit_unary_expr(&self, operator: &Token, right: &Expr) -> CblResult<Object> {
        let r = self.evaluate(right)?;
    
        match operator.type_ {
            TokenType::Bang => match r {
                Object::Bool(r) => Ok(Object::Bool(!r)),
                _ => Err(Error::runtime_error(&format!("Operand must be a bool: {:?}", operator.type_)))
            },
            TokenType::Minus => match r {
                Object::Number(r) => Ok(Object::Number(-r)),
                _ => Err(Error::runtime_error(&format!("Operand must be a number: {:?}", operator.type_))),
            },
            _ => Err(Error::runtime_error(&format!("Unexpected token type: {:?}", operator.type_))),
        }
    }
    
}

impl Interpreter {
    pub fn new() -> Self {
        Interpreter {}
    }

    fn evaluate(&self, expr: &Expr) -> CblResult<Object> {
        expr.accept(self)
    }

    fn is_equal(&self, a: &Object, b: &Object) -> bool {
        match (a, b) {
            (Object::Nil, Object::Nil) => true,
            (Object::Number(a), Object::Number(b)) => a == b,
            (Object::String(a), Object::String(b)) => a == b,
            (Object::Bool(a), Object::Bool(b)) => a == b,
            _ => false,
        }
    }

    pub fn interpret(&self, expr: &Expr) -> CblResult<Object> {
        self.evaluate(expr)
    }
}

#[cfg(test)]
mod tests {
    use crate::{scanner::Scanner, parser::Parser};

    use super::*;

    #[test]
    fn test_interpreter_1() {
        let source = "-17.89 * 391.2";
        let mut scanner = Scanner::new(source);
        let tokens = scanner.scan_tokens();
    
        let mut parser = Parser::new(tokens.clone());
        let expression = parser.parse().unwrap();

        let interpreter = Interpreter::new();
        let result = interpreter.interpret(&expression).unwrap();
        assert_eq!(result, Object::Number(-6998.568_f64));
    }

    #[test]
    fn test_interpreter_2() {
        let source = "\"chess\" + \"rules\"";
        let mut scanner = Scanner::new(source);
        let tokens = scanner.scan_tokens();
    
        let mut parser = Parser::new(tokens.clone());
        let expression = parser.parse().unwrap();

        let interpreter = Interpreter::new();
        let result = interpreter.interpret(&expression).unwrap();
        assert_eq!(result, Object::String("chessrules".to_string()));
    }
}