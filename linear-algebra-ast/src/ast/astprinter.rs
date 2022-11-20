use crate::ast::expression::Binary;
use crate::ast::expression::ExpressionVisitor;
use crate::ast::expression::ExpressionType;
use crate::ast::expression::Literal;
use crate::ast::expression::Unary;
use crate::value::Value;

pub struct ASTPrinter {}
impl ASTPrinter {
    pub fn new() -> Self {
        ASTPrinter {}
    }
    // pub fn print(&mut self, expression: &dyn Expression<Value>) -> Value {
    //     expression.accept(self)
    // }
}

impl ExpressionVisitor<Value> for ASTPrinter {
    fn visit_identifier(&mut self, _identifier: &super::expression::Identifier) -> Value {
        Value::new_scalar(0.0)
    }

    fn visit_literal(&mut self, literal: &Literal) -> Value {
        let value: &Value = &literal.value;
        value.print();
        value.to_owned()
    }

    fn visit_unary(&mut self, unary: &Unary) -> Value {
        print!("({} ", unary.operator.lexeme);
        let value = unary.right.visit(self);
        print!(")");
        value
    }

    fn visit_binary(&mut self, binary: &Binary) -> Value {
        print!("(");
        let left = binary.left.visit(self);
        print!(" {} ", binary.operator.lexeme);
        let _right = binary.right.visit(self);
        print!(")");
        left
    }

    fn visit_grouping(&mut self, grouping: &super::expression::Grouping) -> Value {
        print!("(");
        let value = grouping.expression.visit(self);
        print!(")");
        value
    }
}
