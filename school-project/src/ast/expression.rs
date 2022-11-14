use crate::value::Value;
use crate::tokens::Token;

pub trait ExprVisitor<T> {
    fn visit_literal(&mut self, literal: &Literal) -> T;
    fn visit_unary(&mut self, unary: &Unary) -> T;
    fn visit_binary(&mut self, binary: &Binary) -> T;
    fn visit_grouping(&mut self, grouping: &Grouping) -> T;
}

pub trait Expression<T> {
    fn accept(&self, visitor: &mut dyn ExprVisitor<T>) -> T;
}
pub struct Literal {
    pub value: Value,
}
impl Literal {
    pub fn new(value: Value) -> Self {
        Literal { value }
    }
}
impl Expression<Value> for Literal {
    fn accept(&self, visitor: &mut dyn ExprVisitor<Value>) -> Value {
        visitor.visit_literal(self)
    }
}

pub struct Unary {
    pub operator: Token,
    pub right: Box<dyn Expression<Value>>,
}
impl Unary {
    pub fn new(operator: Token, right: Box<dyn Expression<Value>>) -> Self {
        Unary { operator, right }
    }
}
impl Expression<Value> for Unary {
    fn accept(&self, visitor: &mut dyn ExprVisitor<Value>) -> Value {
        visitor.visit_unary(self)
    }
}

pub struct Binary {
    pub left: Box<dyn Expression<Value>>,
    pub operator:  Token,
    pub right: Box<dyn Expression<Value>>,
}
impl Binary {
    pub fn new(
        left: Box<dyn Expression<Value>>,
        operator: Token,
        right: Box<dyn Expression<Value>>,
    ) -> Self {
        Binary {
            left,
            operator,
            right,
        }
    }
}
impl Expression<Value> for Binary {
    fn accept(&self, visitor: &mut dyn ExprVisitor<Value>) -> Value {
        visitor.visit_binary(self)
    }
}

pub struct Grouping {
    pub expression: Box<dyn Expression<Value>>,
}
impl Grouping {
    pub fn new(expression: Box<dyn Expression<Value>>) -> Self {
        Grouping { expression }
    }
}
impl Expression<Value> for Grouping {
    fn accept(&self, visitor: &mut dyn ExprVisitor<Value>) -> Value {
        self.expression.accept(visitor)
    }
}