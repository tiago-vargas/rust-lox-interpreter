mod token;

use self::token::{Token, Type};

struct Scanner;

impl Scanner {
    fn scan_tokens(source: &str) -> Vec<Token> {
        let mut tokens: Vec<Token> = vec![];

        let mut should_skip = false;
        for (i, byte) in source.as_bytes().iter().enumerate() {
            if should_skip {
                should_skip = false;
                continue;
            }

            let next_byte = source.as_bytes().get(i + 1);
            let token = Self::identify_token(*byte, next_byte);

            match token {
                Some(token) => {
                    if token.is_compound() {
                        should_skip = true;
                    }
                    tokens.push(token);
                }
                None => continue,
            }

        }

        tokens
    }

    fn identify_token(byte: u8, next_byte: Option<&u8>) -> Option<Token> {
        match &[byte] {
            b" " => None,
            b"(" => Some(Token { r#type: Type::LeftParen }),
            b")" => Some(Token { r#type: Type::RightParen }),
            b"{" => Some(Token { r#type: Type::LeftBrace }),
            b"}" => Some(Token { r#type: Type::RightBrace }),
            b"," => Some(Token { r#type: Type::Comma }),
            b"." => Some(Token { r#type: Type::Dot }),
            b"-" => Some(Token { r#type: Type::Minus }),
            b"+" => Some(Token { r#type: Type::Plus }),
            b";" => Some(Token { r#type: Type::Semicolon }),
            b"*" => Some(Token { r#type: Type::Star }),
            b"!" => match next_byte {
                Some(&byte) => {
                    match &[byte] {
                        b"=" => Some(Token { r#type: Type::BangEqual }),
                        b" " => Some(Token { r#type: Type::Bang }),
                        _ => todo!("Got 1 {:?}", std::str::from_utf8(&[byte])),
                    }
                },
                None => Some(Token { r#type: Type::Bang }),
            },
            b">" => match next_byte {
                Some(&byte) => {
                    match &[byte] {
                        b"=" => Some(Token { r#type: Type::GreaterEqual }),
                        b" " => Some(Token { r#type: Type::Greater }),
                        _ => todo!("Got 1 {:?}", std::str::from_utf8(&[byte])),
                    }
                },
                None => Some(Token { r#type: Type::Greater }),
            },
            b"<" => match next_byte {
                Some(&byte) => {
                    match &[byte] {
                        b"=" => Some(Token { r#type: Type::LessEqual }),
                        b" " => Some(Token { r#type: Type::Less }),
                        _ => todo!("Got 1 {:?}", std::str::from_utf8(&[byte])),
                    }
                },
                None => Some(Token { r#type: Type::Less }),
            },
            _ => todo!("Got 2 {:?}", std::str::from_utf8(&[byte])),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scans_simple_unnambiguous_tokens() {
        let code = "(){},.-+;*";

        let tokens = Scanner::scan_tokens(code);

        assert_eq!(
            tokens,
            &[
                Token { r#type: Type::LeftParen },
                Token { r#type: Type::RightParen },
                Token { r#type: Type::LeftBrace },
                Token { r#type: Type::RightBrace },
                Token { r#type: Type::Comma },
                Token { r#type: Type::Dot },
                Token { r#type: Type::Minus },
                Token { r#type: Type::Plus },
                Token { r#type: Type::Semicolon },
                Token { r#type: Type::Star },
            ],
            r#"Did not scan "(){{}},.-+;*""#
        )
    }

    #[test]
    fn scans_ambiguous_tokens() {
        let code = "!= ! > >= < <=";

        let tokens = Scanner::scan_tokens(code);

        assert_eq!(
            tokens,
            &[
                Token { r#type: Type::BangEqual },
                Token { r#type: Type::Bang },
                Token { r#type: Type::Greater },
                Token { r#type: Type::GreaterEqual },
                Token { r#type: Type::Less },
                Token { r#type: Type::LessEqual },
            ],
            r#"Did not scan "!= ! > >= < <=""#
        )
    }
}
