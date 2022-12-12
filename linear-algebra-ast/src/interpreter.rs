use crate::ast::astprinter::ASTPrinter;
use crate::ast::expression::Binary;
use crate::ast::expression::ExpressionType;
use crate::ast::expression::ExpressionVisitor;
use crate::ast::expression::Grouping;
use crate::ast::expression::Identifier;
use crate::ast::expression::Literal;
use crate::ast::expression::Unary;
use crate::ast::statement::ExpressionStatement;
use crate::ast::statement::LetStatement;
use crate::ast::statement::NewLineStatement;
use crate::ast::statement::PrintStatement;
use crate::ast::statement::Statement;
use crate::ast::statement::StatementType;
use crate::ast::statement::StatementVisitor;
use crate::environment::Environment;
use crate::tokens::TokenType;
use crate::value::Value;
use crate::value::ValueType;

pub struct Interpreter {
    pub ast_printer: ASTPrinter,
    pub globals: Environment,
}

impl Interpreter {
    pub fn new() -> Self {
        Interpreter {
            ast_printer: ASTPrinter::new(),
            globals: Environment::new(),
        }
    }
    fn runtime_error(&self, message: &str) {
        println!("Runtime error at {}", message);
        std::process::exit(1);
    }
    pub fn interpret(&mut self, stmts: Vec<Statement>) {
        for statement in stmts {
            statement.visit(self);
        }
    }
}

impl ExpressionVisitor<Value> for Interpreter {
    fn visit_literal(&mut self, literal: &Literal) -> Value {
        literal.value.clone()
    }

    fn visit_identifier(&mut self, identifier: &Identifier) -> Value {
        let value = self.globals.lookup(&identifier.name);
        match value {
            Some(value) => value.clone(),
            None => {
                self.runtime_error("Undefined variable.");
                Value::new_scalar(0.0)
            }
        }
    }

    fn visit_unary(&mut self, unary: &Unary) -> Value {
        let right = unary.right.visit(self);
        match unary.operator.token_type {
            TokenType::TOKEN_MINUS => match right.data {
                ValueType::SCALAR(s) => Value::new_scalar(s.data * -1.0),
                ValueType::MATRIX(m) => {
                    let mut new_matrix = m.clone();
                    new_matrix.scale(-1.0);
                    Value::wrap_matrix(new_matrix)
                }
            },
            TokenType::TOKEN_TRANSPOSE => match right.data {
                ValueType::SCALAR(_) => {
                    self.runtime_error("Cannot transpose a scalar.");
                    Value::new_scalar(0.0)
                }
                ValueType::MATRIX(m) => {
                    let mut new_matrix = m.clone();
                    new_matrix.transpose();
                    Value::wrap_matrix(new_matrix)
                }
            },
            TokenType::TOKEN_REF => match right.data {
                ValueType::SCALAR(_) => {
                    self.runtime_error("Cannot convert scalar to REF matrix");
                    Value::new_scalar(0.0)
                }
                ValueType::MATRIX(m) => {
                    let mut new_matrix = m.clone();
                    new_matrix.ref_matrix();
                    Value::wrap_matrix(new_matrix)
                }
            },
            TokenType::TOKEN_RREF => match right.data {
                ValueType::SCALAR(_) => {
                    self.runtime_error("Cannot convert scalar to RREF matrix");
                    Value::new_scalar(0.0)
                }
                ValueType::MATRIX(m) => {
                    let mut new_matrix = m.clone();
                    new_matrix.rref_matrix();
                    Value::wrap_matrix(new_matrix)
                }
            },
            
            TokenType::TOKEN_INVERSE => match right.data {
                ValueType::SCALAR(_) => {
                    self.runtime_error("Cannot convert scalar to RREF matrix");
                    Value::new_scalar(0.0)
                }
                ValueType::MATRIX(m) => {
                    let mut new_matrix = m.clone();
                    if new_matrix.inverse() {
                        Value::wrap_matrix(new_matrix)
                    } else {
                        self.runtime_error("Matrix is not invertible");
                        Value::new_scalar(0.0)
                    }
                }
            },
            _ => {
                self.runtime_error("Invalid unary operator");
                Value::new_scalar(0.0)
            }
        }
    }
    fn visit_binary(&mut self, binary: &Binary) -> Value {
        let left = binary.left.visit(self);
        let right = binary.right.visit(self);
        match binary.operator.token_type {
            TokenType::TOKEN_PLUS => match left.data {
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
                                    new_matrix.data[i * m.cols + j] += m2.data[i * m.cols + j];
                                }
                            }
                            Value::new_matrix(new_matrix.data, new_matrix.rows, new_matrix.cols)
                        }
                    }
                },
            },
            TokenType::TOKEN_MINUS => match left.data {
                ValueType::SCALAR(s) => match right.data {
                    ValueType::SCALAR(s2) => Value::new_scalar(s.data - s2.data),
                    ValueType::MATRIX(_m) => {
                        self.runtime_error("Cannot subtract a matrix from a scalar");
                        Value::new_scalar(0.0)
                    }
                },
                ValueType::MATRIX(m) => match right.data {
                    ValueType::SCALAR(_s) => {
                        self.runtime_error("Cannot subtract a scalar from a matrix");
                        Value::new_scalar(0.0)
                    }
                    ValueType::MATRIX(m2) => {
                        if m.rows != m2.rows || m.cols != m2.cols {
                            self.runtime_error("Cannot subtract matrices of different sizes");
                            Value::new_scalar(0.0)
                        } else {
                            let mut new_matrix = m.clone();
                            for i in 0..m.rows {
                                for j in 0..m.cols {
                                    new_matrix.data[i * m.cols + j] -= m2.data[i * m.cols + j];
                                }
                            }
                            Value::new_matrix(new_matrix.data, new_matrix.rows, new_matrix.cols)
                        }
                    }
                },
            },
            TokenType::TOKEN_STAR => match left.data {
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
                },
            },
            TokenType::TOKEN_SLASH => match left.data {
                ValueType::SCALAR(s) => match right.data {
                    ValueType::SCALAR(s2) => {
                        if s2.data == 0.0 {
                            self.runtime_error("Cannot divide by zero");
                            Value::new_scalar(0.0)
                        } else {
                            Value::new_scalar(s.data / s2.data)
                        }
                    }
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
                    }
                    ValueType::MATRIX(_m2) => {
                        self.runtime_error("Cannot divide a matrix by a matrix");
                        Value::new_scalar(0.0)
                    }
                },
            },
            TokenType::TOKEN_CARET => match left.data {
                ValueType::SCALAR(s) => match right.data {
                    ValueType::SCALAR(s2) => Value::new_scalar(s.data.powf(s2.data)),
                    ValueType::MATRIX(_m) => {
                        self.runtime_error("Cannot raise a scalar to a matrix");
                        Value::new_scalar(0.0)
                    }
                },
                ValueType::MATRIX(m) => match right.data {
                    ValueType::SCALAR(s) => {
                        if m.rows != m.cols {
                            self.runtime_error("Cannot raise a singular matrix to a power");
                            return Value::new_scalar(0.0);
                        }
                        if s.data < 1.0 {
                            self.runtime_error("Cannot raise a matrix to a power less than 1");
                            return Value::new_scalar(0.0);
                        }
                        if s.data != (s.data as u32) as f64 {
                            self.runtime_error("Cannot raise a matrix to a non-integer power");
                            return Value::new_scalar(0.0);
                        }
                        let power: u32 = s.data as u32;
                        let mut new_matrix = m.clone();
                        for _ in 0..power - 1 {
                            // multiply repeatedly
                            let mut new_matrix2 = vec![0.0; m.rows * m.cols];
                            for i in 0..m.rows {
                                for j in 0..m.cols {
                                    for k in 0..m.cols {
                                        new_matrix2[i * m.cols + j] += new_matrix.data
                                            [i * m.cols + k]
                                            * m.data[k * m.cols + j];
                                    }
                                }
                            }
                            new_matrix.data = new_matrix2;
                        }
                        Value::new_matrix(new_matrix.data, new_matrix.rows, new_matrix.cols)
                    }
                    ValueType::MATRIX(_m2) => {
                        self.runtime_error("Cannot raise a matrix to a matrix");
                        Value::new_scalar(0.0)
                    }
                },
            },
            _ => {
                self.runtime_error("Unknown operator");
                Value::new_scalar(0.0)
            }
        }
    }

    fn visit_grouping(&mut self, grouping: &Grouping) -> Value {
        grouping.expression.visit(self)
    }
}

impl StatementVisitor for Interpreter {
    fn visit_expression_statement(&mut self, statement: &ExpressionStatement) {
        statement.expression.visit(self);
    }

    fn visit_print_statement(&mut self, statement: &PrintStatement) {
        let value = statement.expression.visit(self);
        value.print();
    }

    fn visit_let_statement(&mut self, statement: &LetStatement) {
        let value = statement.initializer.visit(self);
        self.globals
            .define(statement.name.lexeme.to_string(), value);
    }

    fn visit_new_line_statement(&mut self, statement: &NewLineStatement) {
        for _ in 0..statement.lines {
            println!();
        }
    }
}
