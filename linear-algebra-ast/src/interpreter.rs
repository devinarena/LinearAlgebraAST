use crate::ast::astprinter::ASTPrinter;
use crate::ast::expression::Binary;
use crate::ast::expression::ExprVisitor;
use crate::ast::expression::Literal;
use crate::ast::statement::Statement;
use crate::ast::statement::StatementType;
use crate::ast::statement::StmtVisitor;
use crate::tokens::TokenType;
use crate::value::Value;
use crate::value::ValueType;

pub struct Interpreter {
    pub ast_printer: ASTPrinter,
}

impl Interpreter {
    pub fn new() -> Self {
        Interpreter {
            ast_printer: ASTPrinter::new(),
        }
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
                match left.data {
                    ValueType::SCALAR(s) => match right.data {
                        ValueType::SCALAR(s2) => Value::new_scalar(s.data + s2.data),
                        ValueType::MATRIX(_m) => {
                            self.runtime_error("Cannot add a scalar to a matrix");
                            Value::new_scalar(0.0)
                        }
                    },
                    ValueType::MATRIX(m) => match right.data {
                        ValueType::SCALAR(_s) => {
                            self.runtime_error("Cannot add a scalar to a matrix");
                            Value::new_scalar(0.0)
                        }
                        ValueType::MATRIX(m2) => {
                            if m.rows != m2.rows || m.cols != m2.cols {
                                self.runtime_error("Cannot add matrices of different sizes");
                                Value::new_scalar(0.0)
                            } else {
                                let mut new_matrix = m.clone();
                                for i in 0..m.rows {
                                    for j in 0..m.cols {
                                        new_matrix.data[i * m.cols + j] +=
                                            m2.data[i * m.cols + j];
                                    }
                                }
                                Value::new_matrix(new_matrix.data, new_matrix.rows, new_matrix.cols)
                            }
                        }
                    }
                }
            },
            TokenType::TOKEN_MINUS => {
                match left.data {
                    ValueType::SCALAR(s) => match right.data {
                        ValueType::SCALAR(s2) => {
                            Value::new_scalar(s.data - s2.data)
                        },
                        ValueType::MATRIX(_m) => {
                            self.runtime_error("Cannot subtract a matrix from a scalar");
                            Value::new_scalar(0.0)
                        }
                    },
                    ValueType::MATRIX(m) => match right.data {
                        ValueType::SCALAR(_s) => {
                            self.runtime_error("Cannot subtract a scalar from a matrix");
                            Value::new_scalar(0.0)
                        },
                        ValueType::MATRIX(m2) => {
                            if m.rows != m2.rows || m.cols != m2.cols {
                                self.runtime_error("Cannot subtract matrices of different sizes");
                                Value::new_scalar(0.0)
                            } else {
                                let mut new_matrix = m.clone();
                                for i in 0..m.rows {
                                    for j in 0..m.cols {
                                        new_matrix.data[i * m.cols + j] -=
                                            m2.data[i * m.cols + j];
                                    }
                                }
                                Value::new_matrix(new_matrix.data, new_matrix.rows, new_matrix.cols)
                            }
                        }
                    }
                }
            },
            TokenType::TOKEN_STAR => {
                match left.data {
                    ValueType::SCALAR(s) => match right.data {
                        ValueType::SCALAR(s2) => Value::new_scalar(s.data * s2.data),
                        ValueType::MATRIX(m) => {
                            let mut new_matrix = m.clone();
                            for i in 0..m.rows {
                                for j in 0..m.cols {
                                    new_matrix.data[i * m.cols + j] *= s.data;
                                }
                            }
                            Value::new_matrix(new_matrix.data, new_matrix.rows, new_matrix.cols)
                        }
                    },
                    ValueType::MATRIX(m) => match right.data {
                        ValueType::SCALAR(s) => {
                            let mut new_matrix = m.clone();
                            for i in 0..m.rows {
                                for j in 0..m.cols {
                                    new_matrix.data[i * m.cols + j] *= s.data;
                                }
                            }
                            Value::new_matrix(new_matrix.data, new_matrix.rows, new_matrix.cols)
                        }
                        ValueType::MATRIX(m2) => {
                            if m.cols != m2.rows {
                                self.runtime_error("Cannot multiply matrices of different sizes");
                                Value::new_scalar(0.0)
                            } else {
                                let mut new_matrix = vec![0.0; m.rows * m2.cols];
                                for i in 0..m.rows {
                                    for j in 0..m2.cols {
                                        for k in 0..m.cols {
                                            new_matrix[i * m2.cols + j] +=
                                                m.data[i * m.cols + k] * m2.data[k * m2.cols + j];
                                        }
                                    }
                                }
                                Value::new_matrix(new_matrix, m.rows, m2.cols)
                            }
                        }
                    }
                }
            },
            TokenType::TOKEN_SLASH => {
                match left.data {
                    ValueType::SCALAR(s) => match right.data {
                        ValueType::SCALAR(s2) => {
                            if s2.data == 0.0 {
                                self.runtime_error("Cannot divide by zero");
                                Value::new_scalar(0.0)
                            } else {
                                Value::new_scalar(s.data / s2.data)
                            }
                        },
                        ValueType::MATRIX(_m) => {
                            self.runtime_error("Cannot divide a scalar by a matrix");
                            Value::new_scalar(0.0)
                        }
                    },
                    ValueType::MATRIX(m) => match right.data {
                        ValueType::SCALAR(s) => {
                            if s.data == 0.0 {
                                self.runtime_error("Cannot divide by zero");
                                Value::new_scalar(0.0)
                            } else {
                                let mut new_matrix = m.clone();
                                for i in 0..m.rows {
                                    for j in 0..m.cols {
                                        new_matrix.data[i * m.cols + j] /= s.data;
                                    }
                                }
                                Value::new_matrix(new_matrix.data, new_matrix.rows, new_matrix.cols)
                            }
                        },
                        ValueType::MATRIX(_m2) => {
                            self.runtime_error("Cannot divide a matrix by a matrix");
                            Value::new_scalar(0.0)
                        }
                    }
                }
            }
            _ => {
                self.runtime_error("Unknown operator");
                Value::new_scalar(0.0)
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