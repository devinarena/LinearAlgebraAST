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
    content: String,
}

impl Lexer {
    fn number(&self, start: usize, line: i32, tokens: &mut Vec<Token>) -> usize {
        let mut i: usize = start;
        while i < self.content.len() && self.content.chars().nth(i).unwrap().is_numeric() {
            i += 1;
        }
        tokens.push(Token::new(
            TokenType::TOKEN_NUMBER,
            self.content[start..i].to_string(),
            line,
        ));
        i
    }

    fn identifier(&self, start: usize, line: i32, tokens: &mut Vec<Token>) -> usize {
        let mut i: usize = start;
        while i < self.content.len()
            && (self.content.chars().nth(i).unwrap().is_alphabetic()
                || self.content.chars().nth(i).unwrap().is_numeric())
        {
            i += 1;
        }
        let lexeme: String = self.content[start..i].to_string();
        match lexeme {
            // "let" => tokens.push(Token::new(TokenType::TOKEN_LET, lexeme, line)),
            _ => tokens.push(Token::new(TokenType::TOKEN_IDENTIFIER, lexeme, line)),
        }
        i
    }

    pub fn new(file_path: &str) -> Self {
        Lexer {
            content: read_file(file_path),
        }
    }
    pub fn scan_tokens(&self) -> Vec<Token> {
        let mut tokens = Vec::new();
        let mut line = 1;
        let mut index = 0;
        while index < self.content.len() {
            let c = self.content.chars().nth(index).unwrap();
            match c {
                '0'..='9' => index = self.number(index, line, &mut tokens) - 1,
                'a'..='z' | 'A'..='Z' => index = self.identifier(index, line, &mut tokens) - 1,
                '\n' => line += 1,
                ' ' => (),
                '\t' => (),
                '\r' => (),
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
