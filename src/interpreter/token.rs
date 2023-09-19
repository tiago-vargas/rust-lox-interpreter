pub(crate) struct Token {
    pub(crate) r#type: Type,
}

impl PartialEq for Token {
    fn eq(&self, other: &Self) -> bool {
        self.r#type == other.r#type
    }
}

impl std::fmt::Debug for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "⟨{:?}⟩", self.r#type)
    }
}

impl Token {
    pub fn is_compound(&self) -> bool {
        match self.r#type {
            Type::LeftParen
            | Type::RightParen
            | Type::LeftBrace
            | Type::RightBrace
            | Type::Comma
            | Type::Dot
            | Type::Minus
            | Type::Plus
            | Type::Semicolon
            | Type::Star
            | Type::Equal
            | Type::Bang
            | Type::Greater
            | Type::Less
            | Type::StringLiteral(_) => false,  // Not exactly...

            _ => true,
        }
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

    StringLiteral(String),

    Error(Error),
}

#[derive(Debug, PartialEq)]
pub(crate) enum Error {
    UnterminatedString,
}
