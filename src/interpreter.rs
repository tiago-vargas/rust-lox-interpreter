mod token;

use token::{Token, Type};

struct Scanner;

impl Scanner {
    fn scan_tokens(source: &str) -> Vec<Token> {
        let mut tokens: Vec<Token> = vec![];

        let mut should_skip_iteration = false;
        let mut should_skip_line = false;
        for (i, byte) in source.as_bytes().iter().enumerate() {
            if should_skip_iteration {
                should_skip_iteration = false;
                continue;
            }

            if should_skip_line {
                if &[*byte] != b"\n" {
                    should_skip_iteration = true;
                    continue;
                }
                should_skip_line = false;
            }

            let next_byte = source.as_bytes().get(i + 1);
            let token = Self::identify_token(*byte, next_byte);

            match token {
                Token { r#type: Type::SlashSlash } => should_skip_line = true,
                Token { r#type: Type::Whitespace } => continue,
                token => {
                    if token.is_compound() {
                        should_skip_iteration = true;
                    }
                    tokens.push(token);
                }
            }
        }

        tokens
    }

    fn identify_token(byte: u8, next_byte: Option<&u8>) -> Token {
        use Type::*;

        match &[byte] {
            b" "
            | b"\t"
            | b"\r"
            | b"\n" => Token { r#type: Whitespace },
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
            b"!" => decide_token(Bang, (BangEqual, b"="), next_byte),
            b"=" => decide_token(Equal, (EqualEqual, b"="), next_byte),
            b">" => decide_token(Greater, (GreaterEqual, b"="), next_byte),
            b"<" => decide_token(Less, (LessEqual, b"="), next_byte),
            b"/" => decide_token(Slash, (SlashSlash, b"/"), next_byte),
            _ => todo!("Unexpected lexeme {:#?}", std::str::from_utf8(&[byte])),
        }
    }
}

/// # Arguments
/// * `compound_type`: (`type`, `byte`)
///
/// Returns `type` if `next_byte` is `byte`, otherwise returns `simple_type`
fn decide_token(simple_type: Type, compound_type: (Type, &[u8]), next_byte: Option<&u8>) -> Token {
    let expected_bytes = compound_type.1;
    let compound_type = compound_type.0;
    match next_byte {
        Some(&byte) if &[byte] == expected_bytes => Token {
            r#type: compound_type,
        },
        Some(_)
        | None => Token { r#type: simple_type, },
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

    #[test]
    fn scans_ambiguous_tokens() {
        let code = "!= ! == = > >= < <=";

        let tokens = Scanner::scan_tokens(code);

        assert_eq!(
            tokens,
            &[
                Token { r#type: BangEqual },
                Token { r#type: Bang },
                Token { r#type: EqualEqual },
                Token { r#type: Equal },
                Token { r#type: Greater },
                Token { r#type: GreaterEqual },
                Token { r#type: Less },
                Token { r#type: LessEqual },
            ],
            r#"Did not scan "!= ! == = > >= < <=""#
        )
    }

    #[test]
    fn test_different_spacing() {
        let code = "
            (+ -\t*
            =         )
            }\n{
        ";

        let tokens = Scanner::scan_tokens(code);

        assert_eq!(
            tokens,
            &[
                Token { r#type: LeftParen },
                Token { r#type: Plus },
                Token { r#type: Minus },
                Token { r#type: Star },
                Token { r#type: Equal },
                Token { r#type: RightParen },
                Token { r#type: RightBrace },
                Token { r#type: LeftBrace },
            ],
        )
    }

    #[cfg(test)]
    mod comments {
        use super::*;

        #[test]
        fn does_not_scan_comment_glued_to_code() {
            let code = "=//=";

            let tokens = Scanner::scan_tokens(code);

            assert_eq!(
                tokens,
                &[
                    Token { r#type: Equal },
                ],
            )
        }

        #[test]
        fn scans_line_ending_with_comment() {
            let code = "+ - * / =   // This is a comment! != > etc";

            let tokens = Scanner::scan_tokens(code);

            assert_eq!(
                tokens,
                &[
                    Token { r#type: Plus },
                    Token { r#type: Minus },
                    Token { r#type: Star },
                    Token { r#type: Slash },
                    Token { r#type: Equal },
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
                    Token { r#type: Plus },
                    Token { r#type: Minus },
                    Token { r#type: Minus },
                    Token { r#type: Plus },
                ],
            )
        }
    }
}
