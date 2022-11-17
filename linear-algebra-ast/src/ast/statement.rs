
use crate::tokens::Token;
use crate::value::Value;
use crate::ast::expression::Expression;

pub trait StmtVisitor {
    fn visit_expression_statement(&mut self, statement: &ExpressionStatement);
    fn visit_print_statement(&mut self, statement: &PrintStatement);
    fn visit_let_statement(&mut self, statement: &LetStatement);
    fn visit_new_line_statement(&mut self, statement: &NewLineStatement);
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

pub struct LetStatement {
    pub name: Token,
    pub initializer: Box<dyn Expression<Value>>,
}
impl LetStatement {
    pub fn new(name: Token, initializer: Box<dyn Expression<Value>>) -> Self {
        LetStatement { name, initializer }
    }
}
impl StatementType for LetStatement {
    fn accept(&self, visitor: &mut dyn StmtVisitor) -> () {
        visitor.visit_let_statement(self)
    }
}

pub struct NewLineStatement {
    pub lines: usize,
}
impl NewLineStatement {
    pub fn new(lines: usize) -> Self {
        NewLineStatement { lines }
    }
}
impl StatementType for NewLineStatement {
    fn accept(&self, visitor: &mut dyn StmtVisitor) -> () {
        visitor.visit_new_line_statement(self)
    }
}

pub enum Statement {
    Expression(ExpressionStatement),
    Print(PrintStatement),
    Let(LetStatement),
    NewLine(NewLineStatement)
}

impl StatementType for Statement {
    fn accept(&self, visitor: &mut dyn StmtVisitor) -> () {
        match self {
            Statement::Expression(statement) => statement.accept(visitor),
            Statement::Print(statement) => statement.accept(visitor),
            Statement::Let(statement) => statement.accept(visitor),
            Statement::NewLine(statement) => statement.accept(visitor)
        }
    }
}