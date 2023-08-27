use std::fmt::Debug;

struct Scanner;

impl Scanner {
    fn scan_tokens(source: &str) -> &[Token] {
        &[Token {
            r#type: Type::OpenParen,
        }]
    }
}

struct Token {
    r#type: Type,
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
enum Type {
    OpenParen,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scans_open_paren() {
        let code = "(";

        let tokens = Scanner::scan_tokens(code);

        assert_eq!(
            tokens,
            &[Token {
                r#type: Type::OpenParen
            }]
        )
    }
}
