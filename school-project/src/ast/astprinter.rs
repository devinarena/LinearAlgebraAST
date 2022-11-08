use crate::ast::expression::Visitor;
use crate::ast::expression::Literal;

use super::expression::Expression;

pub struct ASTPrinter {

}
impl ASTPrinter {
    pub fn new() -> Self {
        ASTPrinter {}
    }
    pub fn print(&mut self, expression: &dyn Expression<i32>) -> i32 {
        return expression.accept(self)
    }
}

impl Visitor<i32> for ASTPrinter {
    fn visit_literal(&mut self, literal: &Literal) -> i32 {
        literal.value
    }
}