use crate::ast::expression::Binary;
use crate::ast::expression::Unary;
use crate::ast::expression::Expression;
use crate::ast::expression::Literal;
use crate::ast::expression::ExprVisitor;
use crate::value::Value;
use crate::value::ValueType;

pub struct ASTPrinter {}
impl ASTPrinter {
    pub fn new() -> Self {
        ASTPrinter {}
    }
    pub fn print(&mut self, expression: &dyn Expression<Value>) -> Value {
        expression.accept(self)
    }
}

impl ExprVisitor<Value> for ASTPrinter {
    fn visit_literal(&mut self, literal: &Literal) -> Value {
        let value: &Value = &literal.value;
        value.print();
        value.to_owned()
    }

    fn visit_unary(&mut self, unary: &Unary) -> Value {
        print!("({} ", unary.operator.lexeme);
        let value = unary.right.accept(self);
        print!(")");
        value
    }

    fn visit_binary(&mut self, binary: &Binary) -> Value {
        print!("(");
        let left = binary.left.accept(self);
        print!(" {} ", binary.operator.lexeme);
        let _right = binary.right.accept(self);
        print!(")");
        left
    }

    fn visit_grouping(&mut self, grouping: &super::expression::Grouping) -> Value {
        print!("(");
        let value = grouping.expression.accept(self);
        print!(")");
        value
    }
}