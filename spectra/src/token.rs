use std::fmt;

use phf::phf_map;

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub raw: RawToken,
    pub location: Location,
}

impl From<Token> for Precedence {
    fn from(value: Token) -> Self {
        value.raw.into()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Keyword {
    Fun,
    Class,
    While,
    If,
    Else,
    Var,
    Break,
    Continue,
    Return,
}

impl fmt::Display for Keyword {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::Fun => "`fun`",
            Self::Class => "`class`",
            Self::While => "`while`",
            Self::If => "`if`",
            Self::Else => "`else`",
            Self::Var => "`var`",
            Self::Break => "`break`",
            Self::Continue => "`continue`",
            Self::Return => "`return`",
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Punctuation {
    Plus,
    PlusPlus,
    PlusEq,
    Minus,
    MinusMinus,
    MinusEq,
    Star,
    StarStar,
    StarEq,
    Slash,
    SlashEq,
    OpenParent,
    CloseParent,
    OpenBracket,
    CloseBracket,
    OpenBrace,
    CloseBrace,
    Semicolon,
    Comma,
    Dot,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Precedence {
    #[default]
    Lowest,
    Assign,
    Sum,
    Product,
    Power,
    Call,
    FieldAccess,
}

impl From<Punctuation> for Precedence {
    fn from(value: Punctuation) -> Self {
        match value {
            Punctuation::PlusEq
            | Punctuation::MinusEq
            | Punctuation::StarEq
            | Punctuation::SlashEq
            | Punctuation::PlusPlus
            | Punctuation::MinusMinus => Precedence::Assign,
            Punctuation::Plus | Punctuation::Minus => Precedence::Sum,
            Punctuation::Star | Punctuation::Slash => Precedence::Product,
            Punctuation::StarStar => Precedence::Power,
            Punctuation::OpenParent => Precedence::Call,
            Punctuation::Dot => Precedence::FieldAccess,
            _ => Precedence::Lowest,
        }
    }
}

impl fmt::Display for Punctuation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::Plus => "`+`",
            Self::PlusPlus => "`++`",
            Self::PlusEq => "`+=`",
            Self::Minus => "`-`",
            Self::MinusMinus => "`--`",
            Self::MinusEq => "`-=`",
            Self::Star => "`*`",
            Self::StarStar => "`**`",
            Self::StarEq => "`*=`",
            Self::Slash => "`/`",
            Self::SlashEq => "`/=`",
            Self::OpenParent => "`(`",
            Self::CloseParent => "`)`",
            Self::OpenBracket => "`[`",
            Self::CloseBracket => "`]`",
            Self::OpenBrace => "`{`",
            Self::CloseBrace => "`}`",
            Self::Semicolon => "`;`",
            Self::Comma => "`,`",
            Self::Dot => "`.`",
        })
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum RawToken {
    Identifier(String),
    StringLiteral(String),
    Keyword(Keyword),
    Punctuation(Punctuation),
    BoolLiteral(bool),
    IntegerLiteral(u64),
    FloatLiteral(f64),
    CharLiteral(char),
    UnexpectedChar(char),
}

impl fmt::Display for RawToken {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Keyword(keyword) => keyword.fmt(f),
            Self::Identifier(name) => f.write_fmt(format_args!("identifier `{}`", name)),
            Self::StringLiteral(value) => value.fmt(f),
            Self::Punctuation(punctuation) => punctuation.fmt(f),
            Self::BoolLiteral(value) => {
                if *value {
                    f.write_str("`true`")
                } else {
                    f.write_str("`false`")
                }
            }
            Self::IntegerLiteral(value) => value.fmt(f),
            Self::FloatLiteral(value) => value.fmt(f),
            Self::CharLiteral(value) => f.write_fmt(format_args!("'{}'", value)),
            Self::UnexpectedChar(..) => f.write_str("invalid token"),
        }
    }
}

impl From<Punctuation> for RawToken {
    fn from(value: Punctuation) -> Self {
        Self::Punctuation(value)
    }
}

impl From<RawToken> for Precedence {
    fn from(value: RawToken) -> Self {
        match value {
            RawToken::Punctuation(punctuation) => punctuation.into(),
            _ => Precedence::Lowest,
        }
    }
}

pub static KEYWORDS: phf::Map<&'static str, RawToken> = phf_map! {
    "true" => RawToken::BoolLiteral(true),
    "false" => RawToken::BoolLiteral(false),
    "fun" => RawToken::Keyword(Keyword::Fun),
    "class" => RawToken::Keyword(Keyword::Class),
    "while" => RawToken::Keyword(Keyword::While),
    "if" => RawToken::Keyword(Keyword::If),
    "else" => RawToken::Keyword(Keyword::Else),
    "var" => RawToken::Keyword(Keyword::Var),
    "break" => RawToken::Keyword(Keyword::Break),
    "continue" => RawToken::Keyword(Keyword::Continue),
    "return" => RawToken::Keyword(Keyword::Return),
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Location {
    pub start: usize,
    pub end: usize,
}
