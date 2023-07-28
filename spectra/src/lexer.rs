use std::str::Chars;

use crate::token::{Location, Punctuation, RawToken, Token, KEYWORDS};

pub struct Lexer<'s> {
    source: &'s str,
    chars: Chars<'s>,

    offset: usize,

    current: char,
    next: char,
}

impl<'s> Lexer<'s> {
    pub fn new(source: &'s str) -> Self {
        let mut chars = source.chars();

        let current = chars.next().unwrap_or('\0');
        let next = chars.next().unwrap_or('\0');

        Self {
            source,
            chars,
            offset: 0,
            current,
            next,
        }
    }

    fn advance(&mut self) {
        let previous = self.current;

        self.current = self.next;
        self.next = self.chars.next().unwrap_or('\0');

        self.offset += previous.len_utf8();
    }

    #[inline]
    fn advance_twice(&mut self) {
        self.advance();
        self.advance();
    }

    fn eof(&self) -> bool {
        self.current == '\0'
    }

    fn advance_while<F>(&mut self, start_offset: usize, mut f: F) -> &'s str
    where
        F: FnMut(char, char) -> bool,
    {
        while f(self.current, self.next) && !self.eof() {
            self.advance();
        }

        &self.source[start_offset..self.offset]
    }

    fn skip_whitespaces(&mut self) {
        while is_whitespace(self.current) {
            self.advance();
        }
    }

    fn current_char_location(&self) -> Location {
        Location {
            start: self.offset,
            end: self.offset + 1,
        }
    }

    fn location_from(&self, start_offset: usize) -> Location {
        Location {
            start: start_offset,
            end: self.offset,
        }
    }

    fn advance_with(&mut self, raw: impl Into<RawToken>) -> Token {
        let token = Token {
            raw: raw.into(),
            location: self.current_char_location(),
        };

        self.advance();
        token
    }

    fn advance_twice_with(&mut self, raw: impl Into<RawToken>) -> Token {
        let token = Token {
            raw: raw.into(),
            location: Location {
                start: self.offset,
                end: self.offset + 2,
            },
        };

        self.advance_twice();
        token
    }

    fn next_identifier_or_keyword_token(&mut self) -> Token {
        let start_offset = self.offset;
        let identifier_candidate =
            self.advance_while(start_offset, |current, _| is_id_continue(current));

        if let Some(keyword) = KEYWORDS.get(identifier_candidate) {
            Token {
                raw: keyword.clone(),
                location: self.location_from(start_offset),
            }
        } else {
            Token {
                raw: RawToken::Identifier(identifier_candidate.to_owned()),
                location: self.location_from(start_offset),
            }
        }
    }

    // TODO: process floating-point numbers
    fn next_number_token(&mut self) -> Token {
        let start_offset = self.offset;
        let number_string = self.advance_while(start_offset, |current, _| current.is_ascii_digit());

        Token {
            raw: RawToken::IntegerLiteral(number_string.parse().unwrap()),
            location: self.location_from(start_offset),
        }
    }

    fn next_string_token(&mut self) -> Token {
        let start_offset = self.offset;

        let string = self.advance_while(start_offset, |current, _| current != '"');

        Token {
            raw: RawToken::StringLiteral(string.to_owned()),
            location: self.location_from(start_offset),
        }
    }
}

impl Iterator for Lexer<'_> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        self.skip_whitespaces();

        if self.eof() {
            return None;
        }

        Some(match (self.current, self.next) {
            ('+', '+') => self.advance_twice_with(Punctuation::PlusPlus),
            ('+', '=') => self.advance_twice_with(Punctuation::PlusEq),
            ('+', _) => self.advance_with(Punctuation::Plus),
            ('-', '-') => self.advance_twice_with(Punctuation::MinusMinus),
            ('-', '=') => self.advance_twice_with(Punctuation::MinusEq),
            ('-', _) => self.advance_with(Punctuation::Minus),
            ('/', '=') => self.advance_twice_with(Punctuation::SlashEq),
            ('*', '=') => self.advance_twice_with(Punctuation::StarEq),
            ('*', '*') => self.advance_with(Punctuation::StarStar),
            ('*', _) => self.advance_with(Punctuation::Star),
            ('/', _) => self.advance_with(Punctuation::Slash),
            ('(', _) => self.advance_with(Punctuation::OpenParent),
            (')', _) => self.advance_with(Punctuation::CloseParent),
            ('[', _) => self.advance_with(Punctuation::OpenBracket),
            (']', _) => self.advance_with(Punctuation::CloseBracket),
            ('{', _) => self.advance_with(Punctuation::OpenBrace),
            ('}', _) => self.advance_with(Punctuation::CloseBrace),
            (';', _) => self.advance_with(Punctuation::Semicolon),
            (',', _) => self.advance_with(Punctuation::Comma),
            ('.', _) => self.advance_with(Punctuation::Dot),
            ('"', _) => self.next_string_token(),
            (_, _) => {
                if is_id_start(self.current) {
                    self.next_identifier_or_keyword_token()
                } else if self.current.is_ascii_digit() {
                    self.next_number_token()
                } else {
                    Token {
                        raw: RawToken::UnexpectedChar(self.current),
                        location: self.current_char_location(),
                    }
                }
            }
        })
    }
}

pub fn is_whitespace(c: char) -> bool {
    matches!(
        c,
        // Usual ASCII suspects
        '\u{0009}'   // \t
        | '\u{000A}' // \n
        | '\u{000B}' // vertical tab
        | '\u{000C}' // form feed
        | '\u{000D}' // \r
        | '\u{0020}' // space

        // NEXT LINE from latin1
        | '\u{0085}'

        // Bidi markers
        | '\u{200E}' // LEFT-TO-RIGHT MARK
        | '\u{200F}' // RIGHT-TO-LEFT MARK

        // Dedicated whitespace characters from Unicode
        | '\u{2028}' // LINE SEPARATOR
        | '\u{2029}' // PARAGRAPH SEPARATOR
    )
}

fn is_id_start(c: char) -> bool {
    c == '_' || unicode_xid::UnicodeXID::is_xid_start(c)
}

fn is_id_continue(c: char) -> bool {
    unicode_xid::UnicodeXID::is_xid_continue(c)
}

#[cfg(test)]
mod tests {
    use crate::token::{Location, Punctuation, RawToken, Token};

    use super::Lexer;

    #[test]
    fn eof() {
        let mut lexer = Lexer::new("");
        assert_eq!(lexer.next(), None);
    }

    #[test]
    fn identifier() {
        let mut lexer = Lexer::new("test");
        assert_eq!(
            lexer.next(),
            Some(Token {
                raw: RawToken::Identifier("test".to_owned()),
                location: Location { start: 0, end: 4 }
            })
        );
    }

    #[test]
    fn bool() {
        let mut lexer = Lexer::new("true false");

        assert_eq!(
            lexer.next(),
            Some(Token {
                raw: RawToken::BoolLiteral(true),
                location: Location { start: 0, end: 4 }
            })
        );
        assert_eq!(
            lexer.next(),
            Some(Token {
                raw: RawToken::BoolLiteral(false),
                location: Location { start: 5, end: 10 }
            })
        );
    }

    #[test]
    fn punctuation() {
        let mut lexer = Lexer::new("+");

        assert_eq!(
            lexer.next(),
            Some(Token {
                raw: RawToken::Punctuation(Punctuation::Plus),
                location: Location { start: 0, end: 1 }
            })
        );
    }
}
