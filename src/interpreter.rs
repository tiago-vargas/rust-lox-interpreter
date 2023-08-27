mod token;

use self::token::{Token, Type};

struct Scanner;

impl Scanner {
    fn scan_tokens(source: &str) -> Vec<Token> {
        let mut tokens: Vec<Token> = vec![];

        let mut should_skip = false;
        for (i, byte) in source.as_bytes().iter().enumerate() {
            if should_skip {
                continue;
            }

            let next_byte = source.as_bytes().get(i + 1);
            let token = Self::identify_token(*byte, next_byte);
            if token.is_compound() {
                should_skip = true;
            }
            tokens.push(token);
        }

        tokens
    }

    fn identify_token(byte: u8, next_byte: Option<&u8>) -> Token {
        match &[byte] {
            b"(" => Token { r#type: Type::LeftParen },
            b")" => Token { r#type: Type::RightParen },
            b"{" => Token { r#type: Type::LeftBrace },
            b"}" => Token { r#type: Type::RightBrace },
            b"," => Token { r#type: Type::Comma },
            b"." => Token { r#type: Type::Dot },
            b"-" => Token { r#type: Type::Minus },
            b"+" => Token { r#type: Type::Plus },
            b";" => Token { r#type: Type::Semicolon },
            b"*" => Token { r#type: Type::Star },
            b"!" => match next_byte {
                Some(&byte) => {
                    match &[byte] {
                        b"=" => Token { r#type: Type::BangEqual },
                        _ => todo!(),
                    }
                },
                _ => todo!(),
            },
            _ => todo!("Got {:?}", std::str::from_utf8(&[byte])),
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
        let code = "!=";

        let tokens = Scanner::scan_tokens(code);

        assert_eq!(
            tokens,
            &[
                Token { r#type: Type::BangEqual },
            ],
        )
    }
}
