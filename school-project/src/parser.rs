use std::any::TypeId;

use crate::ast::expression::Binary;
use crate::ast::expression::Expression;
use crate::ast::expression::Literal;
use crate::ast::expression::Unary;
use crate::tokens::Token;
use crate::tokens::TokenType;
use crate::value::Scalar;
use crate::value::Value;
use crate::value::ValueType;

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens, current: 0 }
    }

    fn previous(&mut self) -> &Token {
        &self.tokens[self.current - 1]
    }

    fn peek(&mut self) -> &Token {
        &self.tokens[self.current]
    }

    fn is_at_end(&mut self) -> bool {
        self.peek().token_type == TokenType::TOKEN_EOF
    }

    fn advance(&mut self) -> &Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        let token = self.previous();
        token
    }

    fn check(&mut self, token_type: TokenType) -> bool {
        if self.is_at_end() {
            return false;
        }
        self.peek().token_type == token_type
    }

    fn match_token(&mut self, token_type: TokenType) -> bool {
        if self.check(token_type) {
            self.advance();
            return true;
        }
        false
    }

    fn consume(&mut self, token_type: TokenType, message: &str) -> Result<&Token, String> {
        if self.check(token_type) {
            return Ok(self.advance());
        }
        Err(format!("{} at line {}", message, self.peek().line))
    }

    fn matrix(&mut self) -> Box<dyn Expression<Value>> {
        let mut matrix = Vec::new();
        let mut row = Vec::new();
        let mut rows: usize = 0;
        let mut cols: usize = 0;
        while !self.check(TokenType::TOKEN_RIGHT_BRACKET) && !self.is_at_end() {
            if self.check(TokenType::TOKEN_SEMICOLON) {
                if cols == 0 {
                    cols = row.len();
                } else if cols != row.len() {
                    panic!("Invalid matrix");
                }
                rows += 1;
                matrix.push(row);
                row = Vec::new();
                self.advance();
                continue;
            }
            let expr = self.expression();
            // check if literal
            match TypeId::of::<Literal>() == TypeId::of::<expr.as_ref()>() {
                true => {
                    let literal = expr.as_any().downcast_ref::<Literal>().unwrap();
                    row.push(literal.value.clone());
                }
                false => panic!("Invalid matrix"),
            }
        }
        matrix.push(row);
        self.consume(TokenType::TOKEN_RIGHT_BRACKET, "Expected ']' after matrix")?;
        Box::new(Literal::new(Value::new_matrix(matrix, rows, cols)))
    }

    fn literal(&mut self) -> Box<dyn Expression<Value>> {
        self.advance();
        let token = self.previous();
        let value = match token.token_type {
            TokenType::TOKEN_NUMBER => {
                let number: f64 = token.lexeme.parse::<f64>().unwrap();
                Value::new_scalar(number)
            }
            TokenType::TOKEN_LEFT_BRACKET => self.matrix(),
            TokenType::TOKEN_LEFT_PAREN => {
                let expr = self.expression();
                if self
                    .consume(
                        TokenType::TOKEN_RIGHT_PAREN,
                        "Expected ')' after expression",
                    )
                    .is_err()
                {
                    return Box::new(Literal::new(Value::new_scalar(0.0)));
                }
                return expr;
            }
            _ => panic!("Unexpected token: {}", token),
        };
        Box::new(Literal::new(value))
    }

    fn unary(&mut self) -> Box<dyn Expression<Value>> {
        if self.match_token(TokenType::TOKEN_MINUS) {
            let operator = self.previous().clone();
            let right = self.unary();
            return Box::new(Unary::new(operator, right));
        }

        self.literal()
    }

    fn factor(&mut self) -> Box<dyn Expression<Value>> {
        let mut expr = self.unary();

        while self.match_token(TokenType::TOKEN_STAR) || self.match_token(TokenType::TOKEN_SLASH) {
            let operator = self.previous().clone();
            let right = self.factor();
            expr = Box::new(Binary::new(expr, operator, right));
        }
        expr
    }

    fn term(&mut self) -> Box<dyn Expression<Value>> {
        let mut expr = self.factor();

        while self.match_token(TokenType::TOKEN_PLUS) || self.match_token(TokenType::TOKEN_MINUS) {
            let operator = self.previous().clone();
            let right = self.factor();
            expr = Box::new(Binary::new(expr, operator, right));
        }
        expr
    }

    fn comparison(&mut self) -> Box<dyn Expression<Value>> {
        let mut expr = self.term();

        while self.match_token(TokenType::TOKEN_GREATER)
            || self.match_token(TokenType::TOKEN_GREATER_EQUAL)
            || self.match_token(TokenType::TOKEN_LESS)
            || self.match_token(TokenType::TOKEN_LESS_EQUAL)
        {
            let operator = self.previous().clone();
            let right = self.term();
            expr = Box::new(Binary::new(expr, operator, right));
        }

        expr
    }

    fn equality(&mut self) -> Box<dyn Expression<Value>> {
        let mut expr = self.comparison();
        while self.match_token(TokenType::TOKEN_BANG_EQUAL)
            || self.match_token(TokenType::TOKEN_EQUAL_EQUAL)
        {
            let operator = self.previous().clone();
            let right = self.comparison();
            expr = Box::new(Binary::new(expr, operator, right));
        }
        expr
    }

    fn expression(&mut self) -> Box<dyn Expression<Value>> {
        self.equality()
    }

    pub fn parse(&mut self) -> Result<Box<dyn Expression<Value>>, String> {
        let expression = self.expression();
        Ok(expression)
    }
}
