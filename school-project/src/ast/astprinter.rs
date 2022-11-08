use crate::ast::expression::Visitor;
use crate::ast::expression::Literal;

use super::expression::Expression;

pub struct ASTPrinter {

}
impl ASTPrinter {
    pub fn new() -> Self {
        ASTPrinter {}
    }
    pub fn print(&mut self, expression: &dyn Expression<String>) -> String {
        return expression.accept(self)
    }
}

impl Visitor<String> for ASTPrinter {
    fn visit_literal(&mut self, literal: &Literal) -> String {
        literal.value.to_string()
    }
    fn visit_binary(&mut self, binary: &super::expression::Binary) -> String {
        let left = binary.left.accept(self);
        let right = binary.right.accept(self);
        format!("({} {} {})", binary.operator, left, right)
    }
}