use std::fmt::Debug;

pub(crate) struct Token {
    pub(crate) r#type: Type,
}

impl PartialEq for Token {
    fn eq(&self, other: &Self) -> bool {
        self.r#type == other.r#type
    }
}

impl Debug for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "«{:?}»", self.r#type)
    }
}

#[derive(Debug, PartialEq)]
pub(crate) enum Type {
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Star,

    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    Slash,
    SlashSlash,  // Only for internal use
    Whitespace,  // Only for internal use

    String(String),
    Number(Literal),

    Error(Error),

    Keyword(Keyword),
    Identifier(String),
}

#[derive(Debug, PartialEq)]
pub(crate) enum Keyword {
    And,
    Class,
    Else,
    False,
    For,
    Fun,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,
}

#[derive(Debug, PartialEq)]
pub(crate) enum Error {
    UnterminatedString,
}

#[derive(Debug, PartialEq)]
pub(crate) enum Literal {
    Integer(i32),
    Float(f32),
}
