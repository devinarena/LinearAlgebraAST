use crate::ast::expression::Binary;
use crate::ast::expression::Expression;
use crate::ast::expression::Grouping;
use crate::ast::expression::Identifier;
use crate::ast::expression::Literal;
use crate::ast::expression::Unary;
use crate::ast::statement::ExpressionStatement;
use crate::ast::statement::LetStatement;
use crate::ast::statement::NewLineStatement;
use crate::ast::statement::PrintStatement;
use crate::ast::statement::Statement;
use crate::tokens::Token;
use crate::tokens::TokenType;
use crate::value::Matrix;
use crate::value::Value;

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
    error: bool,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser {
            tokens,
            current: 0,
            error: false,
        }
    }

    pub fn new_empty() -> Self {
        Parser {
            tokens: Vec::new(),
            current: 0,
            error: false,
        }
    }

    pub fn set_tokens(&mut self, tokens: Vec<Token>) {
        self.tokens = tokens;
        self.current = 0;
    }

    pub fn parse_error(&mut self, content: &str) {
        let token = self.tokens.get(self.current).unwrap();
        let line = token.line;
        self.error = true;
        println!("Error at line {}: {}", line, content);
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

    fn consume(&mut self, token_type: TokenType, message: &str) -> bool {
        if self.check(token_type) {
            self.advance();
            return true;
        }
        self.parse_error(message);
        return false;
    }

    fn synchronize(&mut self) {
        self.advance();
        while !self.is_at_end() {
            if self.previous().token_type == TokenType::TOKEN_SEMICOLON {
                self.error = false;
                return;
            }
            match self.peek().token_type {
                TokenType::TOKEN_PRINT => {
                    self.error = false;
                    return;
                }
                _ => (),
            }
            self.advance();
        }
    }

    fn matrix(&mut self) -> Expression {
        let mut matrix: Vec<f64> = Vec::new();
        let mut rows: usize = 1;
        let mut cols: usize = 0;
        while !self.check(TokenType::TOKEN_RIGHT_BRACKET) && !self.is_at_end() {
            if self.check(TokenType::TOKEN_PIPE) {
                if cols == 0 {
                    cols = matrix.len();
                } else if matrix.len() % cols != 0 {
                    self.parse_error("Invalid matrix dimensions");
                }
                rows += 1;
                self.advance();
                continue;
            }
            if self.consume(TokenType::TOKEN_NUMBER, "Expected a number") {
                matrix.push(self.previous().lexeme.parse::<f64>().unwrap());
            }
        }
        if cols == 0 {
            cols = matrix.len();
        } else if matrix.len() % cols != 0 {
            self.parse_error("Invalid matrix dimensions");
        }
        if self.consume(TokenType::TOKEN_RIGHT_BRACKET, "Expected ']' after matrix") {
            return Expression::Literal(Literal::new(Value::new_matrix(matrix, rows, cols)));
        } else {
            return Expression::Literal(Literal::new(Value::new_scalar(0.0)));
        }
    }

    fn literal(&mut self) -> Expression {
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
                if !self.consume(
                    TokenType::TOKEN_RIGHT_PAREN,
                    "Expected ')' after expression",
                ) {
                    return Expression::Literal(Literal::new(Value::new_scalar(0.0)));
                }
                return Expression::Grouping(Grouping::new(Box::new(expr)));
            }
            TokenType::TOKEN_IDENTIFIER => {
                let identifier = token.lexeme.clone();
                return Expression::Identifier(Identifier::new(identifier));
            }
            TokenType::TOKEN_IDENTITY => {
                self.consume(
                    TokenType::TOKEN_LEFT_PAREN,
                    "Expect '(' following identity keyword",
                );
                self.consume(
                    TokenType::TOKEN_NUMBER,
                    "Expect size of identity as (n) where n is a positive integer.",
                );
                let rows = self.previous().lexeme.parse::<f64>().unwrap();
                if rows < 0.0 {
                    self.parse_error("Identity cannot be negative sized");
                    return Expression::Literal(Literal::new(Value::new_scalar(0.0)));
                }
                if rows < 1.0 && self.match_token(TokenType::TOKEN_RIGHT_PAREN) {
                    return Expression::Literal(Literal::new(Value::new_scalar(0.0)));
                }
                if rows >= 1.0 && rows == (rows as u32) as f64 {
                    self.consume(
                        TokenType::TOKEN_RIGHT_PAREN,
                        "Expect ')' to close identity matrix dimensions.",
                    );
                    return Expression::Literal(Literal::new(Value::wrap_matrix(Matrix::new_identity(rows as usize))));
                } else {
                    self.parse_error(
                        "Identity should be of size (n) where n is a positive integer.",
                    );
                    return Expression::Literal(Literal::new(Value::new_scalar(0.0)));
                }
            }
            TokenType::TOKEN_REF | TokenType::TOKEN_RREF | TokenType::TOKEN_INVERSE => {
                // type checked at runtime rather than compile time
                // i may need to move the environment
                let operator = self.previous().clone();
                self.consume(
                    TokenType::TOKEN_LEFT_PAREN,
                    "Expect '(' following ref keyword",
                );
                let expr = self.expression();
                self.consume(TokenType::TOKEN_RIGHT_PAREN, "Expect ')' to close ref");
                return Expression::Unary(Unary::new(operator, Box::new(expr)));
            }
            _ => {
                self.parse_error("Unexpected token");
                return Expression::Literal(Literal::new(Value::new_scalar(0.0)));
            }
        };
        Expression::Literal(Literal::new(value))
    }

    fn unary(&mut self) -> Expression {
        if self.match_token(TokenType::TOKEN_MINUS) {
            let operator = self.previous().clone();
            let right = self.unary();
            return Expression::Unary(Unary::new(operator, Box::new(right)));
        }

        self.literal()
    }

    fn exponentiation(&mut self) -> Expression {
        let mut expr = self.unary();

        while self.match_token(TokenType::TOKEN_TRANSPOSE) {
            let operator = self.previous().clone();
            expr = Expression::Unary(Unary::new(operator, Box::new(expr)));
        }

        while self.match_token(TokenType::TOKEN_CARET) {
            let operator = self.previous().clone();
            let right = self.unary();
            expr = Expression::Binary(Binary::new(Box::new(expr), operator, Box::new(right)));
        }

        expr
    }

    fn factor(&mut self) -> Expression {
        let mut expr = self.exponentiation();

        while self.match_token(TokenType::TOKEN_STAR) || self.match_token(TokenType::TOKEN_SLASH) {
            let operator = self.previous().clone();
            let right = self.factor();
            expr = Expression::Binary(Binary::new(Box::new(expr), operator, Box::new(right)));
        }
        expr
    }

    fn term(&mut self) -> Expression {
        let mut expr = self.factor();

        while self.match_token(TokenType::TOKEN_PLUS) || self.match_token(TokenType::TOKEN_MINUS) {
            let operator = self.previous().clone();
            let right = self.factor();
            expr = Expression::Binary(Binary::new(Box::new(expr), operator, Box::new(right)));
        }
        expr
    }

    fn comparison(&mut self) -> Expression {
        let mut expr = self.term();

        while self.match_token(TokenType::TOKEN_GREATER)
            || self.match_token(TokenType::TOKEN_GREATER_EQUAL)
            || self.match_token(TokenType::TOKEN_LESS)
            || self.match_token(TokenType::TOKEN_LESS_EQUAL)
        {
            let operator = self.previous().clone();
            let right = self.term();
            expr = Expression::Binary(Binary::new(Box::new(expr), operator, Box::new(right)));
        }

        expr
    }

    fn equality(&mut self) -> Expression {
        let mut expr = self.comparison();
        while self.match_token(TokenType::TOKEN_BANG_EQUAL)
            || self.match_token(TokenType::TOKEN_EQUAL_EQUAL)
        {
            let operator = self.previous().clone();
            let right = self.comparison();
            expr = Expression::Binary(Binary::new(Box::new(expr), operator, Box::new(right)));
        }
        expr
    }

    fn expression(&mut self) -> Expression {
        self.equality()
    }

    fn print_statement(&mut self) -> Statement {
        let value = self.expression();
        if self.consume(TokenType::TOKEN_SEMICOLON, "Expected ';' after value") {
            return Statement::Print(PrintStatement::new(Box::new(value)));
        } else {
            return Statement::Expression(ExpressionStatement::new(Box::new(Literal::new(
                Value::new_scalar(0.0),
            ))));
        }
    }

    fn expression_statement(&mut self) -> Statement {
        let estmt = Statement::Expression {
            0: ExpressionStatement::new(Box::new(self.expression())),
        };
        self.consume(
            TokenType::TOKEN_SEMICOLON,
            "Expect ';' after expression statement",
        );
        estmt
    }

    fn let_statement(&mut self) -> Statement {
        let mut name = Token::new(TokenType::TOKEN_IDENTIFIER, "".to_string(), 0);
        if self.consume(TokenType::TOKEN_IDENTIFIER, "Expected identifier") {
            name = self.previous().clone();
        }
        if self.consume(TokenType::TOKEN_EQUAL, "Expected '=' after identifier") {
            let value = self.expression();
            if self.consume(TokenType::TOKEN_SEMICOLON, "Expected ';' after value") {
                return Statement::Let(LetStatement::new(name, Box::new(value)));
            } else {
                return Statement::Expression(ExpressionStatement::new(Box::new(Literal::new(
                    Value::new_scalar(0.0),
                ))));
            }
        } else {
            return Statement::Expression(ExpressionStatement::new(Box::new(Literal::new(
                Value::new_scalar(0.0),
            ))));
        }
    }

    fn new_line_statement(&mut self) -> Statement {
        let mut lines: usize = 1;
        if self.match_token(TokenType::TOKEN_NUMBER) {
            let token = self.previous();
            match token.lexeme.parse::<usize>() {
                Ok(n) => lines = n,
                Err(_) => self.parse_error("Expected number after newline"),
            }
        }
        self.consume(TokenType::TOKEN_SEMICOLON, "Expected ';' after newline");
        Statement::NewLine(NewLineStatement::new(lines))
    }

    fn statement(&mut self) -> Statement {
        if self.match_token(TokenType::TOKEN_PRINT) {
            return self.print_statement();
        } else if self.match_token(TokenType::TOKEN_LET) {
            return self.let_statement();
        } else if self.match_token(TokenType::TOKEN_NEWLINE) {
            return self.new_line_statement();
        }

        self.expression_statement()
    }

    pub fn parse(&mut self) -> Result<Vec<Statement>, String> {
        let mut statements: Vec<Statement> = Vec::new();
        while !self.is_at_end() {
            statements.push(self.statement());

            if self.error {
                self.synchronize();
            }
        }
        Ok(statements)
    }
}
