use crate::tokens::Token;
use crate::value::Value;

pub trait ExpressionVisitor<T> {
    fn visit_literal(&mut self, literal: &Literal) -> T;
    fn visit_unary(&mut self, unary: &Unary) -> T;
    fn visit_binary(&mut self, binary: &Binary) -> T;
    fn visit_grouping(&mut self, grouping: &Grouping) -> T;
    fn visit_identifier(&mut self, identifier: &Identifier) -> T;
}

pub trait ExpressionType<T> {
    fn visit(&self, visitor: &mut dyn ExpressionVisitor<T>) -> T;
}
pub struct Literal {
    pub value: Value,
}
impl Literal {
    pub fn new(value: Value) -> Self {
        Literal { value }
    }
}
impl ExpressionType<Value> for Literal {
    fn visit(&self, visitor: &mut dyn ExpressionVisitor<Value>) -> Value {
        visitor.visit_literal(self)
    }
}

pub struct Unary {
    pub operator: Token,
    pub right: Box<Expression>,
}
impl Unary {
    pub fn new(operator: Token, right: Box<Expression>) -> Self {
        Unary { operator, right }
    }
}
impl ExpressionType<Value> for Unary {
    fn visit(&self, visitor: &mut dyn ExpressionVisitor<Value>) -> Value {
        visitor.visit_unary(self)
    }
}

pub struct Binary {
    pub left: Box<Expression>,
    pub operator: Token,
    pub right: Box<Expression>,
}
impl Binary {
    pub fn new(
        left: Box<Expression>,
        operator: Token,
        right: Box<Expression>,
    ) -> Self {
        Binary {
            left,
            operator,
            right,
        }
    }
}
impl ExpressionType<Value> for Binary {
    fn visit(&self, visitor: &mut dyn ExpressionVisitor<Value>) -> Value {
        visitor.visit_binary(self)
    }
}

pub struct Grouping {
    pub expression: Box<Expression>,
}
impl Grouping {
    pub fn new(expression: Box<Expression>) -> Self {
        Grouping { expression }
    }
}
impl ExpressionType<Value> for Grouping {
    fn visit(&self, visitor: &mut dyn ExpressionVisitor<Value>) -> Value {
        self.expression.visit(visitor)
    }
}


pub struct Identifier {
    pub name: String,
}
impl Identifier {
    pub fn new(name: String) -> Self {
        Identifier { name }
    }
}
impl ExpressionType<Value> for Identifier {
    fn visit(&self, visitor: &mut dyn ExpressionVisitor<Value>) -> Value {
        visitor.visit_identifier(self)
    }
}

pub enum Expression {
    Literal(Literal),
    Unary(Unary),
    Binary(Binary),
    Grouping(Grouping),
    Identifier(Identifier),
}
impl<T> ExpressionType<T> for Expression {
    fn visit(&self, visitor: &mut dyn ExpressionVisitor<T>) -> T {
        match self {
            Expression::Literal(literal) => visitor.visit_literal(literal),
            Expression::Unary(unary) => visitor.visit_unary(unary),
            Expression::Binary(binary) => visitor.visit_binary(binary),
            Expression::Grouping(grouping) => visitor.visit_grouping(grouping),
            Expression::Identifier(identifier) => visitor.visit_identifier(identifier),
        }
    }
}
impl<Value> ExpressionType<Value> for Box<Expression> {
    fn visit(&self, visitor: &mut dyn ExpressionVisitor<Value>) -> Value {
        self.as_ref().visit(visitor)
    }
}