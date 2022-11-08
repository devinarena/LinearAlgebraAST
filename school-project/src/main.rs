mod interpreter;
mod lexer;
mod parser;
mod tokens;
mod value;
mod ast {
    pub mod astprinter;
    pub mod expression;
}
use crate::ast::expression::Binary;
use crate::ast::expression::Literal;
use crate::parser::Parser;
use crate::value::Value;
use crate::value::ValueType;
use crate::ast::astprinter::ASTPrinter;
use crate::interpreter::Interpreter;

fn main() {
    let lexer = lexer::Lexer::new("file.m");
    let tokens = lexer.scan_tokens();
    for token in &tokens {
        println!("{}", token);
    }

    let mut parser: Parser = Parser::new(tokens);
    let ast = parser.parse();

    match ast {
        Ok(ast) => {
            let mut ast_printer = ASTPrinter::new();
            ast_printer.print(ast.as_ref());
            let mut interpreter = Interpreter::new();
            println!("");
            match interpreter.interpret(ast.as_ref()).data {
                ValueType::SCALAR(s) => println!("{}", s.data),
                ValueType::MATRIX(_m) => println!("Matrix"),
            }
        }
        Err(error) => {
            println!("{}", error);
        }
    }
}
