pub trait Visitor<T> {
    fn visit_literal(&mut self, literal: &Literal) -> T;
    fn visit_binary(&mut self, binary: &Binary) -> T;
}

pub trait Expression<T> {
    fn accept(&self, visitor: &mut dyn Visitor<T>) -> T;
}

pub struct Literal {
    pub value: i32,
}
impl Literal {
    pub fn new(value: i32) -> Self {
        Literal { value }
    }
}
impl Expression<String> for Literal {
    fn accept(&self, visitor: &mut dyn Visitor<String>) -> String {
        visitor.visit_literal(self)
    }
}

pub struct Binary {
    pub left: Box<dyn Expression<String>>,
    pub operator: char,
    pub right: Box<dyn Expression<String>>,
}
impl Binary {
    pub fn new(
        left: Box<dyn Expression<String>>,
        operator: char,
        right: Box<dyn Expression<String>>,
    ) -> Self {
        Binary {
            left,
            operator,
            right,
        }
    }
}
impl Expression<String> for Binary {
    fn accept(&self, visitor: &mut dyn Visitor<String>) -> String {
        visitor.visit_binary(self)
    }
}
