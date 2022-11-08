use std::fmt;

pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub line: i32,
}

impl Token {
    pub fn new(token_type: TokenType, lexeme: String, line: i32) -> Self {
        Token {
            token_type,
            lexeme,
            line,
        }
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?} - {} - {}", self.token_type, self.lexeme, self.line)
    }
}

#[derive(Debug)]
pub enum TokenType {
    TOKEN_EOF,
    TOKEN_ERROR,
    TOKEN_NUMBER,
    TOKEN_PLUS,
    TOKEN_IDENTIFIER
}
