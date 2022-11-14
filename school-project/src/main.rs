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
use crate::ast::astprinter::ASTPrinter;
use crate::ast::expression::Binary;
use crate::ast::expression::Expression;
use crate::ast::expression::Literal;
use crate::ast::statement;
use crate::ast::statement::Statement;
use crate::interpreter::Interpreter;
use crate::parser::Parser;
use crate::value::Value;
use crate::value::ValueType;

fn main() {
    let lexer = lexer::Lexer::new("file.m");
    let tokens = lexer.scan_tokens();
    for token in &tokens {
        println!("{}", token);
    }

    let mut parser: Parser = Parser::new(tokens);
    let statements = parser.parse();

    let mut ast_printer = ASTPrinter::new();
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
