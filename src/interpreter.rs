mod token;

use token::{Token, Type};

struct Scanner<'a> {
    source: &'a str,
    position: usize,
}

impl Scanner<'_> {
    fn new(source: &str) -> Scanner {
        Scanner { source, position: 0 }
    }

    fn scan_tokens(&mut self) -> Vec<Token> {
        let mut tokens: Vec<Token> = vec![];

        while !self.is_at_end() {
            let r#type = self.identify_token_type();
            let token = Token { r#type };

            match token {
                Token { r#type: Type::SlashSlash } => self.skip_current_line(),
                Token { r#type: Type::Whitespace } => (),
                token => {
                    if token.is_compound() {
                        self.advance();
                    }
                    tokens.push(token);
                }
            }
            self.advance();
        }

        tokens
    }

    fn is_at_end(&self) -> bool {
        self.position >= self.source.len()
    }

    fn current_byte(&self) -> u8 {
        self.source.as_bytes()[self.position]
    }

    fn next_byte(&self) -> Option<&u8> {
        self.source.as_bytes().get(self.position + 1)
    }

    fn advance(&mut self) {
        self.position += 1;
    }

    /// Makes `self.position` be above the first occurrence of some byte in `bytes`.
    fn seek(&mut self, byte: &[u8]) {
        while !self.is_at_end() && byte != [self.current_byte()] {
            self.advance();
        }
    }

    fn skip_current_line(&mut self) {
        self.seek(b"\n");
    }

    fn identify_token_type(&self) -> Type {
        use Type::*;

        match &[self.current_byte()] {
            b" "
            | b"\t"
            | b"\r"
            | b"\n" => Whitespace,
            b"(" => LeftParen,
            b")" => RightParen,
            b"{" => LeftBrace,
            b"}" => RightBrace,
            b"," => Comma,
            b"." => Dot,
            b"-" => Minus,
            b"+" => Plus,
            b";" => Semicolon,
            b"*" => Star,
            b"!" => decide_token_type(Bang, (BangEqual, b"="), self.next_byte()),
            b"=" => decide_token_type(Equal, (EqualEqual, b"="), self.next_byte()),
            b">" => decide_token_type(Greater, (GreaterEqual, b"="), self.next_byte()),
            b"<" => decide_token_type(Less, (LessEqual, b"="), self.next_byte()),
            b"/" => decide_token_type(Slash, (SlashSlash, b"/"), self.next_byte()),
            _ => todo!("Unexpected lexeme {:#?}", std::str::from_utf8(&[self.current_byte()])),
        }
    }
}

/// # Arguments
/// * `compound_type`: (`type`, `byte`)
///
/// Returns `type` if `next_byte` is `byte`, otherwise returns `simple_type`
fn decide_token_type(simple_type: Type, compound_type: (Type, &[u8]), next_byte: Option<&u8>) -> Type {
    let expected_bytes = compound_type.1;
    let compound_type = compound_type.0;
    match next_byte {
        Some(&byte) if &[byte] == expected_bytes => compound_type,
        Some(_)
        | None => simple_type,
    }
}

#[cfg(test)]
mod tests {
    use super::token::Type::*;
    use super::*;

    #[test]
    fn scans_simple_unnambiguous_tokens() {
        let code = "(){},.-+;*";

        let tokens = Scanner::new(code).scan_tokens();

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

        let tokens = Scanner::new(code).scan_tokens();

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

        let tokens = Scanner::new(code).scan_tokens();

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

            let tokens = Scanner::new(code).scan_tokens();

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

            let tokens = Scanner::new(code).scan_tokens();

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

            let tokens = Scanner::new(code).scan_tokens();

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
