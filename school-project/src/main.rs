mod lexer;
mod tokens;
mod ast {
    pub mod expression;
    pub mod astprinter;
}
use crate::ast::expression::Literal;

fn main() {
    let lexer = lexer::Lexer::new("file.m");
    let tokens = lexer.scan_tokens();
    for token in tokens {
        println!("{}", token);
    }
    let ast = Literal::new(1);
    let mut ast_printer = ast::astprinter::ASTPrinter::new();
    println!("{}", ast_printer.print(&ast));
}
