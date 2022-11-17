mod interpreter;
mod lexer;
mod parser;
mod tokens;
mod value;
mod environment;
mod ast {
    pub mod astprinter;
    pub mod expression;
    pub mod statement;
}

use std::io::Write;

use crate::interpreter::Interpreter;
use crate::lexer::Lexer;
use crate::parser::Parser;

fn main() {
    let args = std::env::args().collect::<Vec<String>>();

    if args.len() == 0 {
        println!("Usage: linalg <file>");
        std::process::exit(1);
    } else if args.len() == 1 {
        repl();
        return;
    }

    let file = &args[1];

    let lexer = Lexer::new(file);
    let tokens = lexer.scan_tokens();
    for token in &tokens {
        println!("{}", token);
    }

    let mut parser: Parser = Parser::new(tokens);
    let statements = parser.parse();

    let mut interpreter = Interpreter::new();

    match statements {
        Ok(statements) => {
            interpreter.interpret(statements);
        }
        Err(error) => {
            println!("{}", error);
        }
    }
}

fn repl() {
    let mut lexer = Lexer::new_empty();
    let mut parser = Parser::new_empty();
    let mut interpreter = Interpreter::new();

    loop {
        print!("LA > ");
        let mut input = String::new();
        match std::io::stdout().flush() {
            Ok(_) => {},
            Err(error) => {
                println!("Error flushing stdout: {}", error);
                std::process::exit(1);
            }
        }
        std::io::stdin().read_line(&mut input).unwrap();
        lexer.content = input;
        let tokens = lexer.scan_tokens();
        parser.set_tokens(tokens);
        let statements = parser.parse();
        match statements {
            Ok(statements) => {
                interpreter.interpret(statements);
            }
            Err(error) => {
                println!("{}", error);
            }
        }
    }
}
