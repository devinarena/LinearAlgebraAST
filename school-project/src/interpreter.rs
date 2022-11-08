
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
    fn runtime_error(&self, message: &str) {
        println!("Runtime error at {}", message);
        std::process::exit(1);
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
        match binary.operator {
            '+' => {
                unsafe {
                    Value::new_scalar(left.data.scalar + right.data.scalar)
                }
            }
            '-' => {
                unsafe {
                    Value::new_scalar(left.data.scalar - right.data.scalar)
                }
            }
            '*' => {
                unsafe {
                    Value::new_scalar(left.data.scalar * right.data.scalar)
                }
            }
            '/' => {
                unsafe {
                    if right.data.scalar == 0.0 {
                        self.runtime_error("division by zero");
                    }
                    Value::new_scalar(left.data.scalar / right.data.scalar)
                }
            }
            _ => {
                panic!("Unknown operator: {}", binary.operator);
            }
        }
    }
}