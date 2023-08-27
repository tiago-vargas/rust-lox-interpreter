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
                    r#type: Type::LeftParen,
                });
            }
            b")" => {
                tokens.push(Token {
                    r#type: Type::RightParen,
                });
            }
            b"{" => {
                tokens.push(Token {
                    r#type: Type::LeftBrace,
                });
            }
            b"}" => {
                tokens.push(Token {
                    r#type: Type::RightBrace,
                });
            }
            b"," => {
                tokens.push(Token {
                    r#type: Type::Comma,
                });
            }
            b"." => {
                tokens.push(Token {
                    r#type: Type::Dot,
                });
            }
            b"-" => {
                tokens.push(Token {
                    r#type: Type::Minus,
                });
            }
            b"+" => {
                tokens.push(Token {
                    r#type: Type::Plus,
                });
            }
            b";" => {
                tokens.push(Token {
                    r#type: Type::Semicolon,
                });
            }
            b"*" => {
                tokens.push(Token {
                    r#type: Type::Star,
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scans_right_paren() {
        let code = "(";

        let tokens = Scanner::scan_tokens(code);

        assert_eq!(
            tokens,
            &[Token {
                r#type: Type::LeftParen
            }]
        )
    }

    #[test]
    fn scans_left_paren() {
        let code = ")";

        let tokens = Scanner::scan_tokens(code);

        assert_eq!(
            tokens,
            &[Token {
                r#type: Type::RightParen
            }]
        )
    }

    #[test]
    fn scans_simple_unnambiguous_tokens() {
        let code = "(){},.-+;*";

        let tokens = Scanner::scan_tokens(code);

        assert_eq!(
            tokens,
            &[
                Token {
                    r#type: Type::LeftParen
                },
                Token {
                    r#type: Type::RightParen
                },
                Token {
                    r#type: Type::LeftBrace
                },
                Token {
                    r#type: Type::RightBrace
                },
                Token {
                    r#type: Type::Comma
                },
                Token {
                    r#type: Type::Dot
                },
                Token {
                    r#type: Type::Minus
                },
                Token {
                    r#type: Type::Plus
                },
                Token {
                    r#type: Type::Semicolon
                },
                Token {
                    r#type: Type::Star
                },
            ],
            r#"Did not scan "(){{}},.-+;*""#
        )
    }
}
