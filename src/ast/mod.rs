use crate::token::{Token, TokenType};
use std::any::Any;

fn dummy_identifier() -> Expression {
    Expression::Identifier {
        token: Token {
            literal: "".to_string(),
            t: TokenType::Ident,
        },
        value: "".to_string(),
    }
}

pub trait Node {
    fn string(&self) -> String;
}

pub struct Program {
    pub statements: Vec<Statement>,
}

impl Node for Program {
    fn string(&self) -> String {
        let mut out = String::new();
        for s in &self.statements {
            out.push_str(&s.string());
        }
        out
    }
}

#[derive(Debug, Clone)]
pub enum Statement {
    Let {
        token: Token,
        name: Token,
        value: Expression,
    },
    Return {
        token: Token,
        value: Expression,
    },
    Expression {
        token: Token,
        expression: Expression,
    },
}

impl Node for Statement {
    fn string(&self) -> String {
        match self {
            Statement::Let {
                ref name,
                ref value,
                ..
            } => format!("let {} = {};", &name.literal, value.string()),
            Statement::Return { ref value, .. } => format!("return {};", value.string()),
            Statement::Expression { ref expression, .. } => format!("{};", expression.string()),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Expression {
    Identifier {
        token: Token,
        value: String,
    },
    IntegerLiteral {
        token: Token,
        value: i32,
    },
    Prefix {
        token: Token,
        operator: String,
        right: Box<Expression>,
    },
    Infix {
        token: Token,
        operator: String,
        left: Box<Expression>,
        right: Box<Expression>,
    },
}

impl Node for Expression {
    fn string(&self) -> String {
        use Expression::*;
        match self {
            Identifier { ref value, .. } => value.clone(),
            IntegerLiteral { ref token, .. } => token.literal.clone(),
            Prefix {
                ref operator,
                ref right,
                ..
            } => format!("({}{})", operator, right.string()),
            Infix {
                ref operator,
                ref left,
                ref right,
                ..
            } => format!("({} {} {})", left.string(), operator, right.string()),
        }
    }
}
