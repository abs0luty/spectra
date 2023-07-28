use crate::token::{Location, Token};

pub type Module = Vec<Statement>;
pub type StatementsBlock = Vec<Statement>;

#[derive(Debug, Clone, PartialEq)]
pub enum Statement {
    Expression {
        location: Location,
        expression: Expression,
    },
    Return {
        location: Location,
        return_value: Expression,
    },
    Break {
        location: Location,
    },
    Continue {
        location: Location,
    },
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
        location: Location,
    },
    // a++
    Postfix {
        left: Box<Expression>,
        operator: Token,
        location: Location,
    },
    // !a
    Prefix {
        operator: Token,
        right: Box<Expression>,
        location: Location,
    },
    // a
    Identifier(IdentifierAST),
    // a()
    Call {
        callee: Box<Expression>,
        arguments: Vec<Expression>,
        location: Location,
    },
    // a.b
    FieldAccess {
        left: Box<Expression>,
        right: IdentifierAST,
        location: Location,
    },
}

impl Expression {
    #[inline]
    #[must_use]
    pub const fn location(&self) -> Location {
        match self {
            Self::Identifier(IdentifierAST { location, .. })
            | Self::Prefix { location, .. }
            | Self::Postfix { location, .. }
            | Self::Binary { location, .. }
            | Self::Literal(Literal { location, .. })
            | Self::Call { location, .. }
            | Self::FieldAccess { location, .. } => *location,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Literal {
    pub raw: RawLiteral,
    pub location: Location,
}

#[derive(Debug, Clone, PartialEq)]
pub struct IdentifierAST {
    pub identifier: String,
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
