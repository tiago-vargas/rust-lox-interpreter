#[derive(Debug)]
pub(crate) struct Token {
    pub(crate) r#type: Type,
}

impl PartialEq for Token {
    fn eq(&self, other: &Self) -> bool {
        self.r#type == other.r#type
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
}
