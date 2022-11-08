use std::fmt;

pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub line: usize,
}

impl Token {
    pub fn new(token_type: TokenType, lexeme: String, line: usize) -> Self {
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
#[allow(non_camel_case_types)]
pub enum TokenType {
    TOKEN_EOF,
    TOKEN_ERROR,
    TOKEN_NUMBER,
    TOKEN_PLUS,
    TOKEN_MINUS,
    TOKEN_STAR,
    TOKEN_SLASH,
    TOKEN_IDENTIFIER
}
