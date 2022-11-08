use std::i32;

use crate::value::Value;
use crate::ast::expression::Visitor;
use crate::ast::expression::Literal;
use crate::ast::expression::Expression;
use crate::ast::expression::Binary;


pub struct ASTPrinter {

}
impl ASTPrinter {
    pub fn new() -> Self {
        ASTPrinter {}
    }
    pub fn print(&mut self, expression: &dyn Expression<Value>) -> Value {
        expression.accept(self)
    }
}

impl Visitor<Value> for ASTPrinter {
    fn visit_literal(&mut self, literal: &Literal) -> Value {
        unsafe {
            print!("{}", literal.value.data.int.to_string());
        }
        literal.value
    }
    fn visit_binary(&mut self, binary: &Binary) -> Value {
        print!("(");
        let left = binary.left.accept(self);
        print!(" {} ", binary.operator);
        let _right = binary.right.accept(self);
        print!(")");
        left
    }
}