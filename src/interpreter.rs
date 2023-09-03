mod token;

use self::token::{Token, Type};

struct Scanner<'a> {
    bytes: &'a [u8],
    position: usize,
}

impl Scanner<'_> {
    fn new(source: &str) -> Scanner {
        Scanner { bytes: source.as_bytes(), position: 0 }
    }

    fn scan_tokens(&mut self) -> Vec<Token> {
        let mut tokens: Vec<Token> = vec![];

        while !self.is_at_end() {
            let current_byte = self.current_byte();
            let r#type = self.identify_token(current_byte);
            let token = Token { r#type };

            match token {
                Token { r#type: Type::Whitespace } => {}
                Token { r#type: Type::SlashSlash } => self.skip_current_line(),
                token => tokens.push(token),
            }
            self.advance();
        }

        tokens
    }

    fn is_at_end(&mut self) -> bool {
        self.position >= self.bytes.len()
    }

    fn current_byte(&mut self) -> u8 {
        self.bytes[self.position]
    }

    fn advance(&mut self) {
        self.position += 1;
    }

    fn identify_token(&mut self, byte: u8) -> Type {
        match &[self.current_byte()] {
            b" "
            | b"\t"
            | b"\r"
            | b"\n" => Type::Whitespace,
            b"\"" => {
                let (start, end) = self.measure_string();
                if self.is_at_end() {
                    // Didn't find the closing `"`...
                    Type::Error(token::Error::UnterminatedString)
                } else {
                    let s = String::from_utf8(self.bytes[start+1..end].to_vec());
                    let s = s.unwrap();  // TODO: Add error token
                    Type::String(s)
                }
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
            b"!" => self.decide_token(Type::Bang, (Type::BangEqual, b"=")),
            b"=" => self.decide_token(Type::Equal, (Type::EqualEqual, b"=")),
            b">" => self.decide_token(Type::Greater, (Type::GreaterEqual, b"=")),
            b"<" => self.decide_token(Type::Less, (Type::LessEqual, b"=")),
            b"/" => self.decide_token(Type::Slash, (Type::SlashSlash, b"/")),
            _digit if byte.is_ascii_digit() => {
                let (is_f32, start, end) = self.measure_number();
                let number = std::str::from_utf8(&self.bytes[start..end]);
                if is_f32 {
                    let n = number.unwrap().parse::<f32>().unwrap();
                    Type::Number(token::Literal::Float(n))
                } else {
                    let n = number.unwrap().parse::<i32>().unwrap();
                    Type::Number(token::Literal::Integer(n))
                }
            }
            a if is_ascii_alphabetic(a) => {
                let (start, end) = self.measure_word();
                let word = &self.bytes[start..end];
                match word {
                    b"and" => Type::Keyword(token::Keyword::And),
                    b"class" => Type::Keyword(token::Keyword::Class),
                    b"else" => Type::Keyword(token::Keyword::Else),
                    b"false" => Type::Keyword(token::Keyword::False),
                    b"for" => Type::Keyword(token::Keyword::For),
                    b"fun" => Type::Keyword(token::Keyword::Fun),
                    b"if" => Type::Keyword(token::Keyword::If),
                    b"nil" => Type::Keyword(token::Keyword::Nil),
                    b"or" => Type::Keyword(token::Keyword::Or),
                    b"print" => Type::Keyword(token::Keyword::Print),
                    b"return" => Type::Keyword(token::Keyword::Return),
                    b"super" => Type::Keyword(token::Keyword::Super),
                    b"this" => Type::Keyword(token::Keyword::This),
                    b"true" => Type::Keyword(token::Keyword::True),
                    b"var" => Type::Keyword(token::Keyword::Var),
                    b"while" => Type::Keyword(token::Keyword::While),
                    bytes => Type::Identifier(String::from_utf8(bytes.to_vec()).unwrap()),
                }
            }
            _ => todo!("Unexpected token {:#?}", std::str::from_utf8(&[byte])),
        }
    }

    fn measure_number(&mut self) -> (bool, usize, usize) {
        let mut is_f32 = false;
        let start = self.position;
        self.advance_until_not_ascii_digit();
        if !self.is_at_end() && &[self.current_byte()] == b"." {
            self.advance();
            self.advance_until_not_ascii_digit();
            is_f32 = true;
        }
        let end = self.position;
        (is_f32, start, end)
    }

    fn measure_string(&mut self) -> (usize, usize) {
        let start = self.position;  // Includes the initial `"`
        self.advance();  // Skips the initial `"` again to avoid matching below
        self.advance_until_find_any(&[b"\""]);  // Finds the final `"`
        let end = self.position;  // Also includes the final `"`

        (start, end)
    }

    fn measure_word(&mut self) -> (usize, usize) {
        let start = self.position;
        while !self.is_at_end() && is_ascii_alphabetic(&[self.current_byte()]) {
            self.advance();
        }
        let end = self.position;

        (start, end)
    }

    fn skip_current_line(&mut self) {
        self.advance_until_find_any(&[b"\n"]);
    }

    fn advance_until_find_any(&mut self, bytes: &[&[u8; 1]]) {
        while !self.is_at_end() && !bytes.contains(&&[self.current_byte()]) {
            self.advance();
        }
    }

    fn advance_until_not_ascii_digit(&mut self) {
        while !self.is_at_end() && self.current_byte().is_ascii_digit() {
            self.advance();
        }
    }

    fn decide_token(&mut self, simple_type: Type, compound_type: (Type, &[u8])) -> Type {
        let next_byte = self.bytes.get(self.position + 1);
        match next_byte {
            Some(&byte) => {
                match byte {
                    b if compound_type.1 == &[b] => {
                        self.advance();
                        compound_type.0
                    },
                    _ => simple_type,
                }
            },
            None => simple_type,
        }
    }
}

fn is_ascii_alphabetic(a: &[u8; 1]) -> bool {
    // let byte = a[0];

    (b"a" <= a && a <= b"z") || (b"A" <= a && a <= b"Z")
}


#[cfg(test)]
mod tests {
    use crate::interpreter::token::{Literal, Keyword};

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
    fn detects_unterminated_strings() {
        let code = r#"+ - "This is a string! And it's missing the closing quote..."#;

        let tokens = Scanner::new(code).scan_tokens();

        assert_eq!(
            tokens,
            &[
                Token { r#type: Type::Plus },
                Token { r#type: Type::Minus },
                Token { r#type: Type::Error(token::Error::UnterminatedString) },
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
                Token { r#type: Type::Number(Literal::Integer(123)) },
            ],
        )
    }

    #[test]
    fn scans_integers() {
        let code = "0 + 123 - 1";

        let tokens = Scanner::new(code).scan_tokens();

        assert_eq!(
            tokens,
            &[
                Token { r#type: Type::Number(Literal::Integer(0)) },
                Token { r#type: Type::Plus },
                Token { r#type: Type::Number(Literal::Integer(123)) },
                Token { r#type: Type::Minus },
                Token { r#type: Type::Number(Literal::Integer(1)) },
            ],
        )
    }

    #[test]
    fn scans_lone_floats() {
        let code = "12.3";

        let tokens = Scanner::new(code).scan_tokens();

        assert_eq!(
            tokens,
            &[
                Token { r#type: Type::Number(Literal::Float(12.3)) },
            ],
        )
    }

    #[test]
    fn scans_floats() {
        let code = "0 + 12.3 / 5";

        let tokens = Scanner::new(code).scan_tokens();

        assert_eq!(
            tokens,
            &[
                Token { r#type: Type::Number(Literal::Integer(0)) },
                Token { r#type: Type::Plus },
                Token { r#type: Type::Number(Literal::Float(12.3)) },
                Token { r#type: Type::Slash },
                Token { r#type: Type::Number(Literal::Integer(5)) },
            ],
        )
    }

    #[test]
    fn scans_reserved_words() {
        let code = r#"
            and
            class
            else
            false
            for
            fun
            if
            nil
            or
            print
            return
            super
            this
            true
            var
            while
        "#;

        let tokens = Scanner::new(code).scan_tokens();

        assert_eq!(
            tokens,
            &[
                Token { r#type: Type::Keyword(Keyword::And) },
                Token { r#type: Type::Keyword(Keyword::Class) },
                Token { r#type: Type::Keyword(Keyword::Else) },
                Token { r#type: Type::Keyword(Keyword::False) },
                Token { r#type: Type::Keyword(Keyword::For) },
                Token { r#type: Type::Keyword(Keyword::Fun) },
                Token { r#type: Type::Keyword(Keyword::If) },
                Token { r#type: Type::Keyword(Keyword::Nil) },
                Token { r#type: Type::Keyword(Keyword::Or) },
                Token { r#type: Type::Keyword(Keyword::Print) },
                Token { r#type: Type::Keyword(Keyword::Return) },
                Token { r#type: Type::Keyword(Keyword::Super) },
                Token { r#type: Type::Keyword(Keyword::This) },
                Token { r#type: Type::Keyword(Keyword::True) },
                Token { r#type: Type::Keyword(Keyword::Var) },
                Token { r#type: Type::Keyword(Keyword::While) },
            ],
        )
    }

    #[test]
    fn scans_reserved_words_between_newlines() {
        let code = "fun\nvar";

        let tokens = Scanner::new(code).scan_tokens();

        assert_eq!(
            tokens,
            &[
                Token { r#type: Type::Keyword(Keyword::Fun) },
                Token { r#type: Type::Keyword(Keyword::Var) },
            ],
        )
    }

    // #[test]
    // fn scans_identifiers() {
    //     let code = "var fred = 5;";

    //     let tokens = Scanner::new(code).scan_tokens();

    //     assert_eq!(
    //         tokens,
    //         &[
    //             Token { r#type: Type::Keyword(Keyword::Var) },
    //             Token { r#type: Type::Identifier("fred".to_string()) },
    //             Token { r#type: Type::Equal },
    //             Token { r#type: Type::Number(Literal::Integer(5)) },
    //             Token { r#type: Type::Semicolon },
    //         ],
    //     )
    // }

    // #[test]
    // fn scans_identifiers_with_minimal_whitespace() {
    //     let code = "var fred=5;";

    //     let tokens = Scanner::new(code).scan_tokens();

    //     assert_eq!(
    //         tokens,
    //         &[
    //             Token { r#type: Type::Keyword(Keyword::Var) },
    //             Token { r#type: Type::Identifier("fred".to_string()) },
    //             Token { r#type: Type::Equal },
    //             Token { r#type: Type::Number(Literal::Integer(5)) },
    //             Token { r#type: Type::Semicolon },
    //         ],
    //     )
    // }

    // #[test]
    // fn shows_error_when_source_is_not_in_utf8() {
    //     let code = r#""💖""#.encode_utf16();

    //     let tokens = Scanner::new(code.).scan_tokens();

    //     assert_eq!(
    //         tokens,
    //         &[
    //             Token { r#type: Type::Error },
    //         ],
    //     )
    // }
}
