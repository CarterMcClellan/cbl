use crate::{token::{Token, Object}, error::{CblResult, Error}};

pub enum Expr {
    /// Expressions with 2 operands and 1 operator
    Binary {
        left: Box<Expr>,
        operator: Token,
        right: Box<Expr>,
    },
    /// Grouped expressions like (1 + 2) * 3
    /// useful for overiding precedence
    Grouping { expression: Box<Expr> },
    /// Literal expressions like 1, 2, 3, 4, 5, 6, 7, 8, 9, 0
    Literal { value: Object },
    /// Expressions with a single operator, eg. "-" in "-1"
    Unary { operator: Token, right: Box<Expr> },
}

pub trait Visitor<R> {
    fn visit_binary_expr(&self, left: &Expr, operator: &Token, right: &Expr) -> CblResult<R>;
    fn visit_grouping_expr(&self, expression: &Expr) -> CblResult<R>;
    fn visit_literal_expr(&self, value: &Object) -> CblResult<R>;
    fn visit_unary_expr(&self, operator: &Token, right: &Expr) -> CblResult<R>;
}

impl Expr {
    /// Based on expresion type, call the appropriate visitor method
    pub fn accept<R>(&self, visitor: &dyn Visitor<R>) -> CblResult<R> {
        match self {
            Expr::Binary {
                left,
                operator,
                right,
            } => visitor.visit_binary_expr(left, operator, right),
            Expr::Grouping { expression } => visitor.visit_grouping_expr(expression),
            Expr::Literal { value } => visitor.visit_literal_expr(value),
            Expr::Unary { operator, right } => visitor.visit_unary_expr(operator, right),
        }
    }
}

pub struct AstPrinter;

impl AstPrinter {
    pub fn print(&self, expr: Expr) -> CblResult<String> {
        expr.accept(self)
    }

    fn parenthesize(&self, name: String, exprs: Vec<&Expr>) -> CblResult<String> {
        let mut r = String::new();
        r.push_str("(");
        r.push_str(&name);
        for e in &exprs {
            r.push_str(" ");
            match e.accept(self) {
                Ok(s) => r.push_str(&s),
                Err(e) => return Err(Error::parser_error(&format!("Error: {:?}", e))),
            }
        }
        r.push_str(")");
        Ok(r)
    }
}

impl Visitor<String> for AstPrinter {
    fn visit_binary_expr(&self, left: &Expr, operator: &Token, right: &Expr) -> CblResult<String> {
        self.parenthesize(operator.lexeme.clone(), vec![left, right])
    }

    fn visit_grouping_expr(&self, expr: &Expr) -> CblResult<String> {
        self.parenthesize("group".to_string(), vec![expr])
    }

    fn visit_literal_expr(&self, value: &Object) -> CblResult<String> {
        Ok(value.to_string()) // check for null
    }

    fn visit_unary_expr(&self, operator: &Token, right: &Expr) -> CblResult<String> {
        self.parenthesize(operator.lexeme.clone(), vec![right])
    }
}

#[cfg(test)]
mod tests {
    use crate::token::{Object, TokenType};

    use super::*;

    #[test]
    fn test_ast_printer() {
        let expression = Expr::Binary {
            left: Box::new(Expr::Unary {
                operator: Token::new(TokenType::Minus, "-".to_string(), Object::Nil, 1),
                right: Box::new(Expr::Literal {
                    value: Object::Number(123_f64),
                }),
            }),
            operator: Token::new(TokenType::Star, "*".to_string(), Object::Nil, 1),
            right: Box::new(Expr::Grouping {
                expression: Box::new(Expr::Literal {
                    value: Object::Number(45.67_f64),
                }),
            }),
        };
        let printer = AstPrinter;
        let result = printer.print(expression).unwrap();

        assert_eq!(result, "(* (- 123) (group 45.67))");
    }
}
