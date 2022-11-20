
use crate::tokens::Token;
use crate::value::Value;
use crate::ast::expression::ExpressionType;

pub trait StatementVisitor {
    fn visit_expression_statement(&mut self, statement: &ExpressionStatement);
    fn visit_print_statement(&mut self, statement: &PrintStatement);
    fn visit_let_statement(&mut self, statement: &LetStatement);
    fn visit_new_line_statement(&mut self, statement: &NewLineStatement);
}

pub trait StatementType {
    fn visit(&self, visitor: &mut dyn StatementVisitor) -> ();
}

pub struct ExpressionStatement {
    pub expression: Box<dyn ExpressionType<Value>>,
}
impl ExpressionStatement {
    pub fn new(expression: Box<dyn ExpressionType<Value>>) -> Self {
        ExpressionStatement { expression }
    }
}
impl StatementType for ExpressionStatement {
    fn visit(&self, visitor: &mut dyn StatementVisitor) -> () {
        visitor.visit_expression_statement(self)
    }
}

pub struct PrintStatement {
    pub expression: Box<dyn ExpressionType<Value>>,
}
impl PrintStatement {
    pub fn new(expression: Box<dyn ExpressionType<Value>>) -> Self {
        PrintStatement { expression }
    }
}
impl StatementType for PrintStatement {
    fn visit(&self, visitor: &mut dyn StatementVisitor) -> () {
        visitor.visit_print_statement(self)
    }
}

pub struct LetStatement {
    pub name: Token,
    pub initializer: Box<dyn ExpressionType<Value>>,
}
impl LetStatement {
    pub fn new(name: Token, initializer: Box<dyn ExpressionType<Value>>) -> Self {
        LetStatement { name, initializer }
    }
}
impl StatementType for LetStatement {
    fn visit(&self, visitor: &mut dyn StatementVisitor) -> () {
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
    fn visit(&self, visitor: &mut dyn StatementVisitor) -> () {
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
    fn visit(&self, visitor: &mut dyn StatementVisitor) -> () {
        match self {
            Statement::Expression(statement) => statement.visit(visitor),
            Statement::Print(statement) => statement.visit(visitor),
            Statement::Let(statement) => statement.visit(visitor),
            Statement::NewLine(statement) => statement.visit(visitor)
        }
    }
}