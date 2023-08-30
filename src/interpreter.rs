mod token;

use self::token::{Token, Type};

struct Scanner;

impl Scanner {
    fn scan_tokens(source: &str) -> Vec<Token> {
        let mut tokens: Vec<Token> = vec![];

        let mut should_skip_iteration = false;
        let mut should_skip_line = false;
        let mut position = 0;
        while position < source.as_bytes().len() {
            let byte = source.as_bytes()[position];
            if should_skip_iteration {
                should_skip_iteration = false;
                position += 1;
                continue;
            }

            if should_skip_line {
                if &[byte] != b"\n" {
                    position += 1;
                    continue;
                }
                should_skip_line = false;
            }

            let next_byte = source.as_bytes().get(position + 1);
            let r#type = Self::identify_token(byte, next_byte);
            let token = Token { r#type };

            match token {
                Token { r#type: Type::Whitespace } => {
                    position += 1;
                    continue;
                },
                Token { r#type: Type::SlashSlash } => should_skip_line = true,
                token => {
                    if token.is_compound() {
                        should_skip_iteration = true;
                    }
                    tokens.push(token);
                }
            }
            position += 1;
        }

        tokens
    }

    fn identify_token(byte: u8, next_byte: Option<&u8>) -> Type {
        match &[byte] {
            b" "
            | b"\t"
            | b"\r"
            | b"\n" => Type::Whitespace,
            b"(" => Type::LeftParen,
            b")" => Type::RightParen,
            b"{" => Type::LeftBrace,
            b"}" => Type::RightBrace,
            b"," => Type::Comma,
            b"." => Type::Dot,
            b"-" => Type::Minus,
            b"+" => Type::Plus,
            b";" => Type::Semicolon,
            b"*" => Type::Star,
            b"!" => decide_token(Type::Bang, (Type::BangEqual, b"="), next_byte),
            b"=" => decide_token(Type::Equal, (Type::EqualEqual, b"="), next_byte),
            b">" => decide_token(Type::Greater, (Type::GreaterEqual, b"="), next_byte),
            b"<" => decide_token(Type::Less, (Type::LessEqual, b"="), next_byte),
            b"/" => decide_token(Type::Slash, (Type::SlashSlash, b"/"), next_byte),
            _ => todo!("Unexpected token {:#?}", std::str::from_utf8(&[byte])),
        }
    }
}

fn decide_token(simple_type: Type, compound_type: (Type, &[u8]), next_byte: Option<&u8>) -> Type {
    match next_byte {
        Some(&byte) => {
            match byte {
                b if compound_type.1 == &[b] => compound_type.0,
                _ => simple_type,
            }
        },
        None => simple_type,
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
        let code = "!= ! == = > >= < <=";

        let tokens = Scanner::scan_tokens(code);

        assert_eq!(
            tokens,
            &[
                Token { r#type: Type::BangEqual },
                Token { r#type: Type::Bang },
                Token { r#type: Type::EqualEqual },
                Token { r#type: Type::Equal },
                Token { r#type: Type::Greater },
                Token { r#type: Type::GreaterEqual },
                Token { r#type: Type::Less },
                Token { r#type: Type::LessEqual },
            ],
            r#"Did not scan "!= ! == = > >= < <=""#
        )
    }

    #[test]
    fn scans_ambiguous_tokens_with_comment() {
        let code = "+ - * / =   // This is a comment! != > etc";

        let tokens = Scanner::scan_tokens(code);

        assert_eq!(
            tokens,
            &[
                Token { r#type: Type::Plus },
                Token { r#type: Type::Minus },
                Token { r#type: Type::Star },
                Token { r#type: Type::Slash },
                Token { r#type: Type::Equal },
            ],
            r#"Did not scan "!= ! > >= < <=""#
        )
    }

    #[test]
    fn ignores_whitespace() {
        let code = r#"
            + - * / =
            // This is a comment! != > etc
            "#;

        let tokens = Scanner::scan_tokens(code);

        assert_eq!(
            tokens,
            &[
                Token { r#type: Type::Plus },
                Token { r#type: Type::Minus },
                Token { r#type: Type::Star },
                Token { r#type: Type::Slash },
                Token { r#type: Type::Equal },
            ],
        )
    }

    #[test]
    fn scans_line_after_comment() {
        let code = r#"
            + -
            // This is a comment!
            - +
            "#;

        let tokens = Scanner::scan_tokens(code);

        assert_eq!(
            tokens,
            &[
                Token { r#type: Type::Plus },
                Token { r#type: Type::Minus },
                Token { r#type: Type::Minus },
                Token { r#type: Type::Plus },
            ],
        )
    }
}
