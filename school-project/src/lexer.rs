/**
 * File: lexer.rs
 * Author: Devin Arena
 * Description: Lexer implementation for the AST.
 * Date: 2022-11-07
 */
use std::fs;
use std::process::exit;

use crate::tokens;

pub struct Lexer {
    content: String,
}

impl Lexer {
    pub fn new(file_path: &str) -> Self {
        Lexer {
            content: read_file(file_path),
        }
    }
    pub fn scan_tokens(&self) -> Vec<tokens::Token> {
        let mut tokens = Vec::new();
        let mut line = 1;
        for c in self.content.chars() {
            match c {
                '0'..='9' => tokens.push(tokens::Token::new(
                    tokens::TokenType::TOKEN_NUMBER,
                    c.to_string(),
                    line,
                )),
                '\n' => line += 1,
                _ => (),
            }
        }
        tokens.push(tokens::Token::new(
            tokens::TokenType::TOKEN_EOF,
            "".to_string(),
            line,
        ));
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
