
pub trait Visitor<T> {
    fn visit_expression_statement(&mut self, statement: &ExpressionStatement) -> T;
}

trait Statement<T> {
    fn accept(&self, visitor: &mut T) -> ();
}

pub struct ExpressionStatement {
    pub expression: Box<Expression>,
}
impl ExpressionStatement {
    pub fn new(expression: Box<Expression>) -> Self {
        ExpressionStatement { expression }
    }
}
impl Statement<Interpreter> for ExpressionStatement {
    fn accept(&self, visitor: &mut Interpreter) -> () {
        visitor.visit_expression_statement(self);
    }
}