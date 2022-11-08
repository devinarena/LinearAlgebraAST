
pub trait Visitor<T> {
    fn visit_literal(&mut self, literal: &Literal) -> T;
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
impl Expression<i32> for Literal {
    fn accept(&self, visitor: &mut dyn Visitor<i32>) -> i32 {
        visitor.visit_literal(self)
    }
}