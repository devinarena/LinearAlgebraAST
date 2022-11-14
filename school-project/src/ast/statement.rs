
use crate::value::Value;
use crate::ast::expression::Expression;

pub trait Visitor {
    fn visit_expression_statement(&mut self, statement: &ExpressionStatement);
}

pub trait StatementType {
    fn accept(&self, visitor: &mut dyn Visitor) -> ();
}

pub struct ExpressionStatement {
    pub expression: Box<dyn Expression<Value>>,
}
impl ExpressionStatement {
    pub fn new(expression: Box<dyn Expression<Value>>) -> Self {
        ExpressionStatement { expression }
    }
}
impl StatementType for ExpressionStatement {
    fn accept(&self, visitor: &mut dyn Visitor) -> () {
        visitor.visit_expression_statement(self)
    }
}

pub enum Statement {
    Expression(ExpressionStatement),
}