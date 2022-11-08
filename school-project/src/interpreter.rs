
use crate::value::Value;
use crate::ast::expression::Expression;
use crate::ast::expression::Visitor;
use crate::ast::expression::Literal;
use crate::ast::expression::Binary;

pub struct Interpreter {

}

impl Interpreter {
    pub fn new() -> Self {
        Interpreter {}
    }
}

impl Interpreter {
    pub fn interpret(&mut self, ast: &dyn Expression<Value>) -> Value {
        ast.accept(self)
    }
}

impl Visitor<Value> for Interpreter {
    fn visit_literal(&mut self, literal: &Literal) -> Value {
        literal.value
    }
    fn visit_binary(&mut self, binary: &Binary) -> Value {
        let left = binary.left.accept(self);
        let right = binary.right.accept(self);
        left
    }
}