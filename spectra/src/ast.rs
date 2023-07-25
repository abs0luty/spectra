use crate::token::{Location, RawToken, Token};

pub type StatementsBlock = Vec<Statement>;

#[derive(Debug, Clone, PartialEq)]
pub enum Statement {
    Expression(Expression),
    Return { location: Location, value: Expression },
    Break { location: Location },
    Continue { location: Location },
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
    // 2
    Literal(Literal),
    // a + 2
    Binary {
        left: Box<Expression>,
        right: Box<Expression>,
        operator: Token,
    },
    // a++
    Postfix {
        left: Box<Expression>,
        operator: Token,
    },
    // !a
    Prefix {
        operator: Token,
        right: Box<Expression>,
    },
    // a
    Identifier {
        identifier: String,
        location: Location,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub struct Literal {
    pub raw: RawLiteral,
    pub location: Location,
}

#[derive(Debug, Clone, PartialEq)]
pub enum RawLiteral {
    Integer(u64), 
    Float(f64), 
    String(String), 
    Char(char),
    Bool(bool),
}

