use crate::value::Value;

pub trait Visitor<T> {
    fn visit_literal(&mut self, literal: &Literal) -> T;
    fn visit_binary(&mut self, binary: &Binary) -> T;
}

pub trait Expression<T> {
    fn accept(&self, visitor: &mut dyn Visitor<T>) -> T;
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
    fn accept(&self, visitor: &mut dyn Visitor<Value>) -> Value {
        visitor.visit_literal(self)
    }
}

pub struct Binary {
    pub left: Box<dyn Expression<Value>>,
    pub operator: char,
    pub right: Box<dyn Expression<Value>>,
}
impl Binary {
    pub fn new(
        left: Box<dyn Expression<Value>>,
        operator: char,
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
    fn accept(&self, visitor: &mut dyn Visitor<Value>) -> Value {
        visitor.visit_binary(self)
    }
}