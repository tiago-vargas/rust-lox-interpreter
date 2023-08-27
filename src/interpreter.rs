use std::fmt::Debug;

struct Scanner;

impl Scanner {
    fn scan_tokens(source: &str) -> Vec<Token> {
        let mut tokens: Vec<Token> = vec![];

        for byte in source.as_bytes() {
            Self::scan_token(*byte, &mut tokens);
        }

        tokens
    }

    fn scan_token(byte: u8, tokens: &mut Vec<Token>) {
        match &[byte] {
            b"(" => {
                tokens.push(Token {
                    r#type: Type::OpenParen,
                });
            }
            b")" => {
                tokens.push(Token {
                    r#type: Type::CloseParen,
                });
            }
            _ => todo!()
        }
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
    CloseParen,
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

    #[test]
    fn scans_close_paren() {
        let code = ")";

        let tokens = Scanner::scan_tokens(code);

        assert_eq!(
            tokens,
            &[Token {
                r#type: Type::CloseParen
            }]
        )
    }
}
