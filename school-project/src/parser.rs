
use crate::tokens::Token;
use crate::tokens::TokenType;
use crate::ast::expression::Expression;
use crate::ast::expression::Literal;
use crate::value::Value;
use crate::value::ValueType;
use crate::value::Scalar;

pub struct Parser {
    tokens: Vec<Token>,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser {
            tokens
        }
    }

    fn literal(&mut self) -> Box<dyn Expression<Value>> {
        let token = self.tokens.pop().unwrap();
        let value = match token.token_type {
            TokenType::TOKEN_NUMBER => {
                let number = token.lexeme.parse::<f64>().unwrap();
                Value::new_scalar(number)
            }
            _ => panic!("Unexpected token: {}", token),
        };
        Box::new(Literal::new(value))
    }

    pub fn parse(&mut self) -> Result<Box<dyn Expression<Value>>, String> {
        let expression = self.literal();
        Ok(expression)
    }
}