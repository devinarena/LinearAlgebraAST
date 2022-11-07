use std::fmt::Error;
/**
 * File: lexer.rs
 * Author: Devin Arena
 * Description: Lexer implementation for the AST.
 * Date: 2022-11-07
 */
use std::fs;
use std::process::exit;

pub struct Lexer {
    content: String,
}

impl Lexer {
    pub fn new(file_path: &str) -> Self {
        Lexer {
            content: read_file(file_path),
        }
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
