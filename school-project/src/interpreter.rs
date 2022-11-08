use crate::ast::expression::Binary;
use crate::ast::expression::Expression;
use crate::ast::expression::Literal;
use crate::ast::expression::Visitor;
use crate::value::Value;
use crate::value::ValueType;

pub struct Interpreter {}

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
        literal.value.clone()
    }
    fn visit_binary(&mut self, binary: &Binary) -> Value {
        let left = binary.left.accept(self);
        let right = binary.right.accept(self);
        match binary.operator {
            '+' => {
                if let Value {
                    data: ValueType::SCALAR(s),
                } = left
                {
                    if let Value {
                        data: ValueType::SCALAR(s2),
                    } = right
                    {
                        Value::new_scalar(s.data + s2.data)
                    } else {
                        self.runtime_error("Invalid operand");
                        Value::new_scalar(0.0)
                    }
                } else {
                    self.runtime_error("Invalid operand");
                    Value::new_scalar(0.0)
                }
            }
            '-' => {
                if let Value {
                    data: ValueType::SCALAR(s),
                } = left
                {
                    if let Value {
                        data: ValueType::SCALAR(s2),
                    } = right
                    {
                        Value::new_scalar(s.data - s2.data)
                    } else {
                        self.runtime_error("Invalid operand");
                        Value::new_scalar(0.0)
                    }
                } else {
                    self.runtime_error("Invalid operand");
                    Value::new_scalar(0.0)
                }
            }
            '*' => {
                if let Value {
                    data: ValueType::SCALAR(s),
                } = left
                {
                    if let Value {
                        data: ValueType::SCALAR(s2),
                    } = right
                    {
                        Value::new_scalar(s.data * s2.data)
                    } else {
                        self.runtime_error("Invalid operand");
                        Value::new_scalar(0.0)
                    }
                } else {
                    self.runtime_error("Invalid operand");
                    Value::new_scalar(0.0)
                }
            }
            '/' => {
                if let Value {
                    data: ValueType::SCALAR(s),
                } = left
                {
                    if let Value {
                        data: ValueType::SCALAR(s2),
                    } = right
                    {
                        Value::new_scalar(s.data / s2.data)
                    } else {
                        self.runtime_error("Invalid operand");
                        Value::new_scalar(0.0)
                    }
                } else {
                    self.runtime_error("Invalid operand");
                    Value::new_scalar(0.0)
                }
            }
            _ => {
                panic!("Unknown operator: {}", binary.operator);
            }
        }
    }
}
