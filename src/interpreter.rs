mod token;

use token::{Token, Type};

struct Scanner;

impl Scanner {
    fn scan_tokens(source: &str) -> Vec<Token> {
        let mut tokens: Vec<Token> = vec![];

        for &byte in source.as_bytes() {
            let token = Self::identify_token(byte);
            tokens.push(token);
        }

        tokens
    }

    fn identify_token(byte: u8) -> Token {
        use Type::*;

        match &[byte] {
            b"(" => Token { r#type: LeftParen },
            b")" => Token { r#type: RightParen },
            b"{" => Token { r#type: LeftBrace },
            b"}" => Token { r#type: RightBrace },
            b"," => Token { r#type: Comma },
            b"." => Token { r#type: Dot },
            b"-" => Token { r#type: Minus },
            b"+" => Token { r#type: Plus },
            b";" => Token { r#type: Semicolon },
            b"*" => Token { r#type: Star },
            _ => todo!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::token::Type::*;
    use super::*;

    #[test]
    fn scans_simple_unnambiguous_tokens() {
        let code = "(){},.-+;*";

        let tokens = Scanner::scan_tokens(code);

        assert_eq!(
            tokens,
            &[
                Token { r#type: LeftParen },
                Token { r#type: RightParen },
                Token { r#type: LeftBrace },
                Token { r#type: RightBrace },
                Token { r#type: Comma },
                Token { r#type: Dot },
                Token { r#type: Minus },
                Token { r#type: Plus },
                Token { r#type: Semicolon },
                Token { r#type: Star },
            ],
            r#"Did not scan "(){{}},.-+;*""#
        )
    }
}
