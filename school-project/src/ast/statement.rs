
use crate::value::Value;
use crate::ast::expression::Expression;

pub trait StmtVisitor {
    fn visit_expression_statement(&mut self, statement: &ExpressionStatement);
    fn visit_print_statement(&mut self, statement: &PrintStatement);
}

pub trait StatementType {
    fn accept(&self, visitor: &mut dyn StmtVisitor) -> ();
}

pub struct ExpressionStatement {
    pub expression: Box<dyn Expression<Value>>,
}
impl ExpressionStatement {
    pub fn new(expression: Box<dyn Expression<Value>>) -> Self {
        ExpressionStatement { expression }
    }
}
impl StatementType for ExpressionStatement {
    fn accept(&self, visitor: &mut dyn StmtVisitor) -> () {
        visitor.visit_expression_statement(self)
    }
}

pub struct PrintStatement {
    pub expression: Box<dyn Expression<Value>>,
}
impl PrintStatement {
    pub fn new(expression: Box<dyn Expression<Value>>) -> Self {
        PrintStatement { expression }
    }
}
impl StatementType for PrintStatement {
    fn accept(&self, visitor: &mut dyn StmtVisitor) -> () {
        visitor.visit_print_statement(self)
    }
}

pub enum Statement {
    Expression(ExpressionStatement),
    Print(PrintStatement),
}

impl StatementType for Statement {
    fn accept(&self, visitor: &mut dyn StmtVisitor) -> () {
        match self {
            Statement::Expression(statement) => statement.accept(visitor),
            Statement::Print(statement) => statement.accept(visitor),
        }
    }
}