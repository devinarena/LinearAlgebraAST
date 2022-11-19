/**
 * File: lexer.rs
 * Author: Devin Arena
 * Description: Lexer implementation for the AST.
 * Date: 2022-11-07
 */
use std::fs;
use std::process::exit;

use crate::tokens::Token;
use crate::tokens::TokenType;

pub struct Lexer {
    pub content: String,
}

impl Lexer {
    fn number(&self, start: usize, line: usize, tokens: &mut Vec<Token>) -> usize {
        let mut i: usize = start;
        while i < self.content.len() && self.content.chars().nth(i).unwrap().is_numeric() {
            i += 1;
            if i < self.content.len() && self.content.chars().nth(i).unwrap() == '.' {
                i += 1;
                while i < self.content.len() && self.content.chars().nth(i).unwrap().is_numeric() {
                    i += 1;
                }
                break;
            }
        }
        tokens.push(Token::new(
            TokenType::TOKEN_NUMBER,
            self.content[start..i].to_string(),
            line,
        ));
        i
    }

    fn identifier(&self, start: usize, line: usize, tokens: &mut Vec<Token>) -> usize {
        let mut i: usize = start;
        while i < self.content.len()
            && (self.content.chars().nth(i).unwrap().is_alphabetic()
                || self.content.chars().nth(i).unwrap().is_numeric())
        {
            i += 1;
        }
        let lexeme: String = self.content[start..i].to_string();
        match lexeme.as_bytes() {
            b"let" => tokens.push(Token::new(TokenType::TOKEN_LET, lexeme, line)),
            b"print" => tokens.push(Token::new(TokenType::TOKEN_PRINT, lexeme, line)),
            b"newline" => tokens.push(Token::new(TokenType::TOKEN_NEWLINE, lexeme, line)),
            _ => tokens.push(Token::new(TokenType::TOKEN_IDENTIFIER, lexeme, line)),
        }
        i
    }

    pub fn new(file_path: &str) -> Self {
        Lexer {
            content: read_file(file_path),
        }
    }

    pub fn new_empty() -> Self {
        Lexer {
            content: "".to_string()
        }
    }

    pub fn scan_tokens(&self) -> Vec<Token> {
        let mut tokens = Vec::new();
        let mut line: usize = 1;
        let mut index: usize = 0;
        while index < self.content.len() {
            let c = self.content.chars().nth(index).unwrap();
            match c {
                '0'..='9' => index = self.number(index, line, &mut tokens) - 1,
                'a'..='z' | 'A'..='Z' => index = self.identifier(index, line, &mut tokens) - 1,
                '\n' => line += 1,
                ' ' => (),
                '\t' => (),
                '\r' => (),
                '+' => tokens.push(Token::new(TokenType::TOKEN_PLUS, c.to_string(), line)),
                '-' => tokens.push(Token::new(TokenType::TOKEN_MINUS, c.to_string(), line)),
                '*' => tokens.push(Token::new(TokenType::TOKEN_STAR, c.to_string(), line)),
                '/' => tokens.push(Token::new(TokenType::TOKEN_SLASH, c.to_string(), line)),
                '^' => tokens.push(Token::new(TokenType::TOKEN_CARET, c.to_string(), line)),
                '=' => {
                    if self.content.chars().nth(index + 1).unwrap() == '=' {
                        tokens.push(Token::new(
                            TokenType::TOKEN_EQUAL_EQUAL,
                            "==".to_string(),
                            line,
                        ));
                        index += 1;
                    } else {
                        tokens.push(Token::new(TokenType::TOKEN_EQUAL, c.to_string(), line));
                    }
                }
                '!' => {
                    if self.content.chars().nth(index + 1).unwrap() == '=' {
                        tokens.push(Token::new(
                            TokenType::TOKEN_BANG_EQUAL,
                            "!=".to_string(),
                            line,
                        ));
                        index += 1;
                    } else {
                        tokens.push(Token::new(TokenType::TOKEN_BANG, c.to_string(), line));
                    }
                }
                '<' => {
                    if self.content.chars().nth(index + 1).unwrap() == '=' {
                        tokens.push(Token::new(
                            TokenType::TOKEN_LESS_EQUAL,
                            "<=".to_string(),
                            line,
                        ));
                        index += 1;
                    } else {
                        tokens.push(Token::new(TokenType::TOKEN_LESS, c.to_string(), line));
                    }
                }
                '>' => {
                    if self.content.chars().nth(index + 1).unwrap() == '=' {
                        tokens.push(Token::new(
                            TokenType::TOKEN_GREATER_EQUAL,
                            ">=".to_string(),
                            line,
                        ));
                        index += 1;
                    } else {
                        tokens.push(Token::new(TokenType::TOKEN_GREATER, c.to_string(), line));
                    }
                }
                '(' => tokens.push(Token::new(TokenType::TOKEN_LEFT_PAREN, c.to_string(), line)),
                ')' => tokens.push(Token::new(
                    TokenType::TOKEN_RIGHT_PAREN,
                    c.to_string(),
                    line,
                )),
                '{' => tokens.push(Token::new(TokenType::TOKEN_LEFT_BRACE, c.to_string(), line)),
                '}' => tokens.push(Token::new(
                    TokenType::TOKEN_RIGHT_BRACE,
                    c.to_string(),
                    line,
                )),
                '[' => tokens.push(Token::new(
                    TokenType::TOKEN_LEFT_BRACKET,
                    c.to_string(),
                    line,
                )),
                ']' => tokens.push(Token::new(
                    TokenType::TOKEN_RIGHT_BRACKET,
                    c.to_string(),
                    line,
                )),
                ',' => tokens.push(Token::new(TokenType::TOKEN_COMMA, c.to_string(), line)),
                '.' => tokens.push(Token::new(TokenType::TOKEN_DOT, c.to_string(), line)),
                ';' => tokens.push(Token::new(TokenType::TOKEN_SEMICOLON, c.to_string(), line)),
                '|' => tokens.push(Token::new(TokenType::TOKEN_PIPE, c.to_string(), line)),
                _ => tokens.push(Token::new(
                    TokenType::TOKEN_ERROR,
                    "Unknown token: ".to_string() + &c.to_string(),
                    line,
                )),
            }
            index += 1;
        }
        tokens.push(Token::new(TokenType::TOKEN_EOF, "".to_string(), line));
        tokens
    }
}

fn read_file(file_path: &str) -> String {
    let data = fs::read_to_string(file_path);
    match data {
        Ok(data) => {
            return data;
        }
        Err(error) => {
            println!("Failed to read file: {}", error);
            exit(1);
        }
    }
}
