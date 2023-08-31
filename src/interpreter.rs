mod token;

use self::token::{Token, Type};

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

        while self.position < self.source.len() {
            let byte = self.source.as_bytes()[self.position];

            let next_byte = self.source.as_bytes().get(self.position + 1);
            let r#type = self.identify_token(byte, next_byte);
            let token = Token { r#type };

            match token {
                Token { r#type: Type::Whitespace } => {}
                Token { r#type: Type::SlashSlash } => self.skip_current_line(),
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

    fn advance(&mut self) {
        self.position += 1;
    }

    fn identify_token(&mut self, byte: u8, next_byte: Option<&u8>) -> Type {
        match &[byte] {
            b" "
            | b"\t"
            | b"\r"
            | b"\n" => Type::Whitespace,
            b"\"" => {
                self.advance();  // Skips the initial `"`
                let start = self.position;
                self.advance_until_find_any(&[b"\""]);  // Finds the final `"`
                let end = self.position;
                let s = String::from_utf8(self.source.as_bytes()[start..end].to_vec());
                let s = s.unwrap_or("".to_string());  // TODO: Add error token
                Type::String(s)
            }
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
            _digit if byte.is_ascii_digit() => {
                let start = self.position;
                self.advance_until_find_any(&[b" ", b"\n"]);
                self.advance();
                let end = self.position;
                let n = std::str::from_utf8(&self.source.as_bytes()[start..end]).unwrap().parse::<i32>().unwrap();
                Type::Number(n)
            }
            _ => todo!("Unexpected token {:#?}", std::str::from_utf8(&[byte])),
        }
    }

    fn skip_current_line(&mut self) {
        self.advance_until_find_any(&[b"\n"]);
    }

    fn advance_until_find_any(&mut self, bytes: &[&[u8; 1]]) {
        while !bytes.contains(&&[self.source.as_bytes()[self.position]]) && self.position < self.source.len() - 1 {
            self.advance();
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

        let tokens = Scanner::new(code).scan_tokens();

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

        let tokens = Scanner::new(code).scan_tokens();

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

        let tokens = Scanner::new(code).scan_tokens();

        assert_eq!(
            tokens,
            &[
                Token { r#type: Type::Plus },
                Token { r#type: Type::Minus },
                Token { r#type: Type::Star },
                Token { r#type: Type::Slash },
                Token { r#type: Type::Equal },
            ],
            r#"+ - * / =   // This is a comment! != > etc"#
        )
    }

    #[test]
    fn ignores_whitespace() {
        let code = r#"
            + - * / =
            // This is a comment! != > etc
            "#;

        let tokens = Scanner::new(code).scan_tokens();

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

        let tokens = Scanner::new(code).scan_tokens();

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

    #[test]
    fn scans_lone_strings() {
        let code = r#""This is a string!""#;

        let tokens = Scanner::new(code).scan_tokens();

        assert_eq!(
            tokens,
            &[
                Token { r#type: Type::String("This is a string!".to_string()) },
            ],
        )
    }

    #[test]
    fn scans_strings() {
        let code = r#"+ - "This is a string!" - +"#;

        let tokens = Scanner::new(code).scan_tokens();

        assert_eq!(
            tokens,
            &[
                Token { r#type: Type::Plus },
                Token { r#type: Type::Minus },
                Token { r#type: Type::String("This is a string!".to_string()) },
                Token { r#type: Type::Minus },
                Token { r#type: Type::Plus },
            ],
        )
    }

    #[test]
    fn scans_multiline_strings() {
        let code = r#"
        + - "This is a string!
        And it is still going!"
        - +"#;

        let tokens = Scanner::new(code).scan_tokens();

        assert_eq!(
            tokens,
            &[
                Token { r#type: Type::Plus },
                Token { r#type: Type::Minus },
                Token { r#type: Type::String("This is a string!\n        And it is still going!".to_string()) },
                Token { r#type: Type::Minus },
                Token { r#type: Type::Plus },
            ],
        )
    }

    #[test]
    fn scans_lone_integers() {
        let code = "123";

        let tokens = Scanner::new(code).scan_tokens();

        assert_eq!(
            tokens,
            &[
                Token { r#type: Type::Number(123) },
            ],
        )
    }
}
