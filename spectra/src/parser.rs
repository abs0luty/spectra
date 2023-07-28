use crate::{
    ast::{Expression, IdentifierAST, Literal, Module, RawLiteral, Statement, StatementsBlock},
    lexer::Lexer,
    token::{Keyword, Location, Precedence, Punctuation, RawToken, Token},
};
use std::iter::Peekable;

pub struct Parser<'s> {
    lexer: Peekable<Lexer<'s>>,
}

impl<'s> Parser<'s> {
    pub fn new(source: &'s str) -> Self {
        Self::from(Lexer::new(source))
    }

    pub fn from(lexer: Lexer<'s>) -> Self {
        Self {
            lexer: lexer.peekable(),
        }
    }

    pub fn consume(&mut self, expected: impl Into<RawToken>) -> ParseResult<()> {
        self.consume_and_return(expected).map(|_| ())
    }

    pub fn consume_identifier(&mut self) -> ParseResult<IdentifierAST> {
        if let Some(got) = self.lexer.next() {
            if let RawToken::Identifier(identifier) = got.raw {
                Ok(IdentifierAST {
                    identifier,
                    location: got.location,
                })
            } else {
                Err(ParseError {
                    expected: "identifier".to_owned(),
                    got: Some(got.clone()),
                })
            }
        } else {
            Err(ParseError {
                expected: "identifier".to_owned(),
                got: None,
            })
        }
    }

    pub fn consume_and_return(&mut self, expected: impl Into<RawToken>) -> ParseResult<Token> {
        let expected = expected.into();

        if let Some(got) = self.lexer.next() {
            if got.raw == expected.clone().into() {
                Ok(got.clone())
            } else {
                Err(ParseError {
                    expected: expected.to_string(),
                    got: Some(got.clone()),
                })
            }
        } else {
            Err(ParseError {
                expected: expected.to_string(),
                got: None,
            })
        }
    }

    pub fn parse_expression(&mut self, precedence: Precedence) -> ParseResult<Expression> {
        let mut left = self.parse_primary_expression()?;

        while precedence
            < self
                .lexer
                .peek()
                .map(|t| t.clone().into())
                .unwrap_or(Precedence::Lowest)
        {
            left = match self.lexer.next() {
                Some(
                    operator @ Token {
                        raw:
                            RawToken::Punctuation(
                                Punctuation::Plus
                                | Punctuation::Minus
                                | Punctuation::Star
                                | Punctuation::Slash,
                            ),
                        ..
                    },
                ) => {
                    let right = self.parse_expression(operator.raw.clone().into())?;

                    Expression::Binary {
                        location: Location {
                            start: left.location().start,
                            end: right.location().end,
                        },
                        left: Box::new(left),
                        right: Box::new(right),
                        operator,
                    }
                }
                Some(
                    operator @ Token {
                        raw: RawToken::Punctuation(Punctuation::PlusPlus | Punctuation::MinusMinus),
                        ..
                    },
                ) => Expression::Postfix {
                    location: Location {
                        start: left.location().start,
                        end: operator.location.end,
                    },
                    left: Box::new(left),
                    operator,
                },
                Some(Token {
                    raw: RawToken::Punctuation(Punctuation::Dot),
                    ..
                }) => {
                    let right = self.consume_identifier()?;

                    Expression::FieldAccess {
                        location: Location {
                            start: left.location().start,
                            end: right.location.end,
                        },
                        left: Box::new(left),
                        right,
                    }
                }
                Some(Token {
                    raw: RawToken::Punctuation(Punctuation::OpenParent),
                    ..
                }) => {
                    let mut arguments = vec![];

                    while self
                        .lexer
                        .peek()
                        .is_some_and(|token| token.raw != RawToken::from(Punctuation::CloseParent))
                    {
                        arguments.push(self.parse_expression(Precedence::Lowest)?);

                        if self
                            .lexer
                            .peek()
                            .is_some_and(|token| token.raw == RawToken::from(Punctuation::Comma))
                        {
                            self.lexer.next();
                        } else {
                            break;
                        }
                    }

                    Expression::Call {
                        location: Location {
                            start: left.location().start,
                            end: self
                                .consume_and_return(Punctuation::CloseParent)?
                                .location
                                .end,
                        },
                        callee: Box::new(left),
                        arguments,
                    }
                }
                _ => break,
            }
        }

        Ok(left)
    }

    fn parse_primary_expression(&mut self) -> ParseResult<Expression> {
        match self.lexer.next() {
            Some(Token {
                raw: RawToken::Punctuation(Punctuation::OpenParent),
                ..
            }) => {
                let inner = self.parse_expression(Precedence::Lowest)?;
                self.consume(Punctuation::CloseParent)?;

                Ok(inner)
            }
            Some(Token {
                raw: RawToken::Identifier(identifier),
                location,
            }) => Ok(Expression::Identifier(IdentifierAST {
                identifier,
                location,
            })),
            Some(Token {
                raw: RawToken::IntegerLiteral(value),
                location,
            }) => Ok(Expression::Literal(Literal {
                raw: RawLiteral::Integer(value),
                location,
            })),
            Some(Token {
                raw: RawToken::BoolLiteral(value),
                location,
            }) => Ok(Expression::Literal(Literal {
                raw: RawLiteral::Bool(value),
                location,
            })),
            Some(Token {
                raw: RawToken::StringLiteral(value),
                location,
            }) => Ok(Expression::Literal(Literal {
                raw: RawLiteral::String(value),
                location,
            })),
            Some(Token {
                raw: RawToken::CharLiteral(value),
                location,
            }) => Ok(Expression::Literal(Literal {
                raw: RawLiteral::Char(value),
                location,
            })),
            Some(Token {
                raw: RawToken::Keyword(Keyword::Fun),
                location: Location { start, .. },
            }) => {
                self.consume(Punctuation::OpenParent)?;

                let mut parameters = vec![];

                while self
                    .lexer
                    .peek()
                    .is_some_and(|token| token.raw != RawToken::from(Punctuation::CloseParent))
                {
                    parameters.push(self.consume_identifier()?);

                    if self
                        .lexer
                        .peek()
                        .is_some_and(|token| token.raw == RawToken::from(Punctuation::Comma))
                    {
                        self.lexer.next();
                    } else {
                        break;
                    }
                }

                self.consume(Punctuation::CloseParent)?;

                let block = self.parse_statements_block()?;

                Ok(Expression::Function {
                    location: Location {
                        start,
                        end: block.location.end,
                    },
                    parameters,
                    block,
                })
            }
            got => {
                return Err(ParseError {
                    expected: "expression".to_owned(),
                    got,
                })
            }
        }
    }

    pub fn parse_statement(&mut self) -> ParseResult<Statement> {
        match self.lexer.peek() {
            Some(Token {
                raw: RawToken::Keyword(Keyword::Continue),
                location,
            }) => {
                let start = location.start;
                self.lexer.next();

                Ok(Statement::Continue {
                    location: Location {
                        start,
                        end: self
                            .consume_and_return(Punctuation::Semicolon)?
                            .location
                            .end,
                    },
                })
            }
            Some(Token {
                raw: RawToken::Keyword(Keyword::Break),
                location,
            }) => {
                let start = location.start;
                self.lexer.next();

                Ok(Statement::Break {
                    location: Location {
                        start,
                        end: self
                            .consume_and_return(Punctuation::Semicolon)?
                            .location
                            .end,
                    },
                })
            }
            Some(Token {
                raw: RawToken::Keyword(Keyword::Return),
                location,
            }) => {
                let start = location.start;
                self.lexer.next();
                let return_value = self.parse_expression(Precedence::Lowest)?;

                Ok(Statement::Return {
                    location: Location {
                        start,
                        end: self
                            .consume_and_return(Punctuation::Semicolon)?
                            .location
                            .end,
                    },
                    return_value,
                })
            }
            Some(Token {
                raw: RawToken::Keyword(Keyword::Var),
                location,
            }) => {
                let start = location.start;
                self.lexer.next();

                let name = self.consume_identifier()?;

                self.consume(Punctuation::Eq)?;

                let value = self.parse_expression(Precedence::Lowest)?;

                Ok(Statement::Var {
                    location: Location {
                        start,
                        end: self
                            .consume_and_return(Punctuation::Semicolon)?
                            .location
                            .end,
                    },
                    name,
                    value,
                })
            }
            _ => {
                let expression = self.parse_expression(Precedence::Lowest)?;

                let result = Ok(Statement::Expression {
                    location: Location {
                        start: expression.location().start,
                        end: self
                            .consume_and_return(Punctuation::Semicolon)?
                            .location
                            .end,
                    },
                    expression,
                });

                result
            }
        }
    }

    pub fn parse_statements_block(&mut self) -> ParseResult<StatementsBlock> {
        let start = self
            .consume_and_return(Punctuation::OpenBrace)?
            .location
            .start;

        let mut statements = vec![];

        while self
            .lexer
            .peek()
            .is_some_and(|token| token.raw != RawToken::from(Punctuation::CloseBrace))
        {
            statements.push(self.parse_statement()?);
        }

        Ok(StatementsBlock {
            location: Location {
                start,
                end: self
                    .consume_and_return(Punctuation::CloseBrace)?
                    .location
                    .end,
            },
            statements,
        })
    }

    pub fn parse(&mut self) -> ParseResult<Module> {
        let mut statements = vec![];

        while self.lexer.peek().is_some() {
            statements.push(self.parse_statement()?);
        }

        Ok(statements)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ParseError {
    pub expected: String,
    pub got: Option<Token>,
}

pub type ParseResult<T> = Result<T, ParseError>;
