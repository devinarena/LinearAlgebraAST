use crate::ast::expression::Binary;
use crate::ast::expression::Expression;
use crate::ast::expression::Literal;
use crate::ast::expression::Visitor;
use crate::tokens::Token;
use crate::tokens::TokenType;
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
    fn visit_unary(&mut self, unary: &crate::ast::expression::Unary) -> Value {
        let right = unary.right.accept(self);
        match unary.operator.token_type {
            TokenType::TOKEN_MINUS => match right.data {
                ValueType::SCALAR(s) => Value::new_scalar(s.data * -1.0),
                ValueType::MATRIX(_m) => {
                    self.runtime_error("Cannot negate a matrix");
                    Value::new_scalar(0.0)
                }
            },
            _ => {
                self.runtime_error("Invalid unary operator");
                Value::new_scalar(0.0)
            }
        }
    }
    fn visit_binary(&mut self, binary: &Binary) -> Value {
        let left = binary.left.accept(self);
        let right = binary.right.accept(self);
        match binary.operator.token_type {
            TokenType::TOKEN_PLUS => {
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
            TokenType::TOKEN_MINUS => {
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
            TokenType::TOKEN_STAR => {
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
            TokenType::TOKEN_SLASH => {
                if let Value {
                    data: ValueType::SCALAR(s),
                } = left
                {
                    if let Value {
                        data: ValueType::SCALAR(s2),
                    } = right
                    {
                        if s2.data == 0.0 {
                            self.runtime_error("Cannot divide by zero");
                            Value::new_scalar(0.0)
                        } else {
                            Value::new_scalar(s.data / s2.data)
                        }
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

    fn visit_grouping(&mut self, grouping: &crate::ast::expression::Grouping) -> Value {
        grouping.expression.accept(self)
    }
}
