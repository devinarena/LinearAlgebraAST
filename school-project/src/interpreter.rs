use crate::ast::expression::Binary;
use crate::ast::expression::ExprVisitor;
use crate::ast::expression::Expression;
use crate::ast::expression::Literal;
use crate::ast::statement;
use crate::ast::statement::Statement;
use crate::ast::statement::StatementType;
use crate::ast::statement::StmtVisitor;
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
    pub fn interpret(&mut self, stmts: Vec<Statement>) {
        for statement in stmts {
            statement.accept(self);
        }
    }
}

impl ExprVisitor<Value> for Interpreter {
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
                        // can only be a matrix
                        if let Value {
                            data: ValueType::MATRIX(m),
                        } = right
                        {
                            let mut matrix = m.clone();
                            matrix.scale(s.data);
                            Value {
                                data: ValueType::MATRIX(matrix),
                            }
                        } else {
                            self.runtime_error("Invalid operand");
                            Value::new_scalar(0.0)
                        }
                    }
                } else {
                    if let Value {
                        data: ValueType::MATRIX(m),
                    } = left
                    {
                        if let Value {
                            data: ValueType::SCALAR(s2),
                        } = right
                        {
                            let mut matrix = m.clone();
                            matrix.scale(s2.data);
                            Value {
                                data: ValueType::MATRIX(matrix),
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

impl StmtVisitor for Interpreter {
    fn visit_expression_statement(
        &mut self,
        statement: &crate::ast::statement::ExpressionStatement,
    ) {
        statement.expression.accept(self);
    }

    fn visit_print_statement(&mut self, statement: &crate::ast::statement::PrintStatement) {
        let value = statement.expression.accept(self);
        value.print();
    }
}
