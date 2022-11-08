use crate::ast::expression::Binary;
use crate::ast::expression::Expression;
use crate::ast::expression::Literal;
use crate::ast::expression::Visitor;
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

impl Visitor<Value> for ASTPrinter {
    fn visit_literal(&mut self, literal: &Literal) -> Value {
        let value: &Value = &literal.value;
        match value.data.clone() {
            ValueType::SCALAR(s) => {
                print!("{}", s.data);
            }
            ValueType::MATRIX(m) => {
                print!("(");
                for i in 0..m.rows {
                    for j in 0..m.cols {
                        print!("{}", m.data[i * m.cols + j]);
                        if j != m.cols - 1 {
                            print!(",");
                        }
                    }
                    if i != m.rows - 1 {
                        print!(";");
                    }
                }
                print!(")");
            }
        }
        value.to_owned()
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
