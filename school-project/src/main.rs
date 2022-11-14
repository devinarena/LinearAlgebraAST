mod interpreter;
mod lexer;
mod parser;
mod tokens;
mod value;
mod ast {
    pub mod astprinter;
    pub mod expression;
    pub mod statement;
}
use crate::interpreter::Interpreter;
use crate::parser::Parser;

fn main() {
    let args = std::env::args().collect::<Vec<String>>();

    if args.len() == 0 {
        println!("Usage: linalg <file>");
        std::process::exit(1);
    } if args.len() == 1 {
        return;
    }

    let file = &args[1];

    let lexer = lexer::Lexer::new(file);
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