mod lexer;
mod interpreter;
mod tokens;
mod value;
mod ast {
    pub mod astprinter;
    pub mod expression;
}
use crate::ast::expression::Binary;
use crate::ast::expression::Literal;

fn main() {
    let lexer = lexer::Lexer::new("file.m");
    let tokens = lexer.scan_tokens();
    for token in tokens {
        println!("{}", token);
    }
    let left: Literal = Literal::new(value::Value::new_int(1));
    let mut right: Box<Literal> = Box::new(Literal::new(value::Value::new_int(2)));
    let lbin: Binary = Binary::new(Box::new(left), '+', right);
    right = Box::new(Literal::new(value::Value::new_int(3)));
    let ast: Binary = Binary::new(Box::new(lbin), '*', right);
    let mut ast_printer = ast::astprinter::ASTPrinter::new(); 
    ast_printer.print(&ast);
    // let mut interpreter = interpreter::Interpreter::new();
    // println!("{}", interpreter.interpret(&ast));
}
