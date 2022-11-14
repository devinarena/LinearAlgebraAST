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

    match statements {
        Ok(statements) => {
            for stmt in statements {
                match stmt {
                    Statement::Expression(expr) => {
                        let mut ast_printer = ASTPrinter::new();
                        ast_printer.print(expr.expression.as_ref());
                        let mut interpreter = Interpreter::new();
                        let value = interpreter.interpret(expr.expression.as_ref());
                        value.print();
                    }
                    _ => {
                        println!("Statement invalid!");
                    }
                }
            }
        }
        Err(error) => {
            println!("{}", error);
        }
    }
}
