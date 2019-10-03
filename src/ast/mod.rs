use crate::token::Token;
use std::any::Any;

pub struct Program {
    pub statements: Vec<Statement>,
}

#[derive(Debug, Clone)]
pub enum Statement {
    Let {
        token: Token,
        name: Token,
        //        value: Expression,
    },
    Return {
        token: Token,
        //        value: Expression
    },
}

#[derive(Debug, Clone)]
pub enum Expression {
    Identifier { token: Token, value: String },
}
