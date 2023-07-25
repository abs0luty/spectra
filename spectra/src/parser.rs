use crate::{
    ast::{Expression, Literal, RawLiteral, Statement, StatementsBlock},
    lexer::Lexer,
    token::{Keyword, Precedence, Punctuation, RawToken, Token},
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

    pub fn consume_and_return(&mut self, expected: impl Into<RawToken>) -> ParseResult<Token> {
        let expected = expected.into();

        if let Some(got) = self.lexer.peek() {
            if got.raw == expected.clone().into() {
                Ok(got.clone())
            } else {
                Err(ParseError {
                    expected: expected.to_string(),
                    got: Some(got.clone()),
                })
            }
        } else {
            todo!()
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
            left = match self.lexer.peek() {
                Some(Token {
                    raw:
                        RawToken::Punctuation(
                            Punctuation::Plus
                            | Punctuation::Minus
                            | Punctuation::Star
                            | Punctuation::Slash,
                        ),
                    ..
                }) => {
                    let operator = self.lexer.next().unwrap();

                    let right = self.parse_expression(operator.raw.clone().into())?;

                    Expression::Binary {
                        left: Box::new(left),
                        right: Box::new(right),
                        operator,
                    }
                }
                Some(Token {
                    raw: RawToken::Punctuation(Punctuation::PlusPlus | Punctuation::MinusMinus),
                    ..
                }) => {
                    let operator = self.lexer.next().unwrap();

                    Expression::Postfix {
                        left: Box::new(left),
                        operator,
                    }
                }
                _ => break,
            }
        }

        Ok(left)
    }

    fn parse_primary_expression(&mut self) -> ParseResult<Expression> {
        let next = self.lexer.next();

        match next {
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
            }) => Ok(Expression::Identifier {
                identifier,
                location,
            }),
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
                ..
            }) => Ok(Statement::Continue {
                location: self.consume_and_return(Punctuation::Semicolon)?.location,
            }),
            Some(Token {
                raw: RawToken::Keyword(Keyword::Break),
                ..
            }) => Ok(Statement::Break {
                location: self.consume_and_return(Punctuation::Semicolon)?.location,
            }),
            _ => Ok(Statement::Expression(
                self.parse_expression(Precedence::Lowest)?,
            )),
        }
    }

    pub fn parse_statements_block(&mut self) -> ParseResult<StatementsBlock> {
        self.consume(Punctuation::OpenBrace)?;

        let mut statements = vec![];

        while self
            .lexer
            .peek()
            .is_some_and(|token| token.raw == Punctuation::CloseBrace.into())
        {
            statements.push(self.parse_statement()?);
        }

        self.consume(Punctuation::CloseBrace)?;

        Ok(statements)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ParseError {
    pub expected: String,
    pub got: Option<Token>,
}

pub type ParseResult<T> = Result<T, ParseError>;

#[cfg(test)]
mod tests {
    use crate::{
        ast::{Expression, Literal, RawLiteral},
        token::{Location, Precedence, Punctuation, RawToken, Token},
    };

    use super::Parser;

    #[test]
    fn identifier_expression() {
        let mut parser = Parser::new("a");

        assert_eq!(
            parser.parse_expression(Precedence::Lowest),
            Ok(Expression::Identifier {
                identifier: "a".to_owned(),
                location: Location { start: 0, end: 1 },
            })
        );
    }

    #[test]
    fn binary_expression1() {
        let mut parser = Parser::new("a + b");

        assert_eq!(
            parser.parse_expression(Precedence::Lowest),
            Ok(Expression::Binary {
                left: Box::new(Expression::Identifier {
                    identifier: "a".to_owned(),
                    location: Location { start: 0, end: 1 }
                }),
                right: Box::new(Expression::Identifier {
                    identifier: "b".to_owned(),
                    location: Location { start: 4, end: 5 }
                }),
                operator: Token {
                    raw: RawToken::Punctuation(Punctuation::Plus),
                    location: Location { start: 2, end: 3 }
                }
            })
        );
    }

    #[test]
    fn binary_expression2() {
        let mut parser = Parser::new("a + b - c");

        assert_eq!(
            parser.parse_expression(Precedence::Lowest),
            Ok(Expression::Binary {
                left: Box::new(Expression::Binary {
                    left: Box::new(Expression::Identifier {
                        identifier: "a".to_owned(),
                        location: Location { start: 0, end: 1 }
                    }),
                    right: Box::new(Expression::Identifier {
                        identifier: "b".to_owned(),
                        location: Location { start: 4, end: 5 }
                    }),
                    operator: Token {
                        raw: RawToken::Punctuation(Punctuation::Plus),
                        location: Location { start: 2, end: 3 }
                    }
                }),
                right: Box::new(Expression::Identifier {
                    identifier: "c".to_owned(),
                    location: Location { start: 8, end: 9 }
                }),
                operator: Token {
                    raw: RawToken::Punctuation(Punctuation::Minus),
                    location: Location { start: 6, end: 7 }
                }
            })
        );
    }

    #[test]
    fn binary_expression3() {
        let mut parser = Parser::new("a + b * c");

        assert_eq!(
            parser.parse_expression(Precedence::Lowest),
            Ok(Expression::Binary {
                left: Box::new(Expression::Identifier {
                    identifier: "a".to_owned(),
                    location: Location { start: 0, end: 1 }
                }),
                right: Box::new(Expression::Binary {
                    left: Box::new(Expression::Identifier {
                        identifier: "b".to_owned(),
                        location: Location { start: 4, end: 5 }
                    }),
                    right: Box::new(Expression::Identifier {
                        identifier: "c".to_owned(),
                        location: Location { start: 8, end: 9 }
                    }),
                    operator: Token {
                        raw: RawToken::Punctuation(Punctuation::Star),
                        location: Location { start: 6, end: 7 }
                    }
                }),
                operator: Token {
                    raw: RawToken::Punctuation(Punctuation::Plus),
                    location: Location { start: 2, end: 3 }
                }
            })
        );
    }
}
