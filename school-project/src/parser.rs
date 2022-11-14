use std::any::Any;

use crate::ast::expression::Binary;
use crate::ast::expression::Expression;
use crate::ast::expression::Literal;
use crate::ast::expression::Unary;
use crate::ast::statement::ExpressionStatement;
use crate::ast::statement::PrintStatement;
use crate::ast::statement::Statement;
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
        let mut matrix: Vec<f64> = Vec::new();
        let mut rows: usize = 1;
        let mut cols: usize = 0;
        while !self.check(TokenType::TOKEN_RIGHT_BRACKET) && !self.is_at_end() {
            if self.check(TokenType::TOKEN_SEMICOLON) {
                if cols == 0 {
                    cols = matrix.len();
                } else if matrix.len() % cols != 0 {
                    panic!("Invalid matrix");
                }
                rows += 1;
                self.advance();
                continue;
            }
            match self.consume(TokenType::TOKEN_NUMBER, "Expected a number") {
                Ok(number) => {
                    matrix.push(number.lexeme.parse::<f64>().unwrap());
                }
                Err(e) => {
                    panic!("{}", e);
                }
            }
        }
        match self.consume(TokenType::TOKEN_RIGHT_BRACKET, "Expected ']' after matrix") {
            Ok(_) => return Box::new(Literal::new(Value::new_matrix(matrix, rows, cols))),
            Err(e) => {
                panic!("{}", e);
            }
        }
    }

    fn literal(&mut self) -> Box<dyn Expression<Value>> {
        self.advance();
        let token = self.previous();
        let value = match token.token_type {
            TokenType::TOKEN_NUMBER => {
                let number: f64 = token.lexeme.parse::<f64>().unwrap();
                Value::new_scalar(number)
            }
            TokenType::TOKEN_LEFT_BRACKET => return self.matrix(),
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

    fn print_statement(&mut self) -> Statement {
        let value = self.expression();
        match self.consume(TokenType::TOKEN_SEMICOLON, "Expected ';' after value") {
            Ok(_) => Statement::Print {
                0: PrintStatement::new(value),
            },
            Err(e) => {
                panic!("{}", e);
            }
        }
    }

    fn expression_statement(&mut self) -> Statement {
        let estmt = Statement::Expression {
            0: ExpressionStatement::new(self.expression()),
        };
        self.consume(
            TokenType::TOKEN_SEMICOLON,
            "Expect ';' after expression statement",
        );
        estmt
    }

    fn statement(&mut self) -> Statement {
        if self.match_token(TokenType::TOKEN_PRINT) {
            return self.print_statement();
        }

        self.expression_statement()
    }

    pub fn parse(&mut self) -> Result<Vec<Statement>, String> {
        let mut statements: Vec<Statement> = Vec::new();
        while !self.is_at_end() {
            statements.push(self.statement());
        }
        Ok(statements)
    }
}
