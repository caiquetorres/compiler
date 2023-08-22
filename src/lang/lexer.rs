use super::{kind::Kind, token::Token};

// As a vector we should tokenize the values when creating or something like that before allowing the structure to be iterated. That way we can add the EndOfFileToken.

pub struct Lexer {
    current_position: usize,
    text: String,
}

impl Lexer {
    pub fn new(text: &str) -> Self {
        Self {
            current_position: 0,
            text: String::from(text),
        }
    }

    pub fn next(&mut self) -> Token {
        if self.current_char() == '\0' {
            return Token::new(Kind::EndOfFileToken, "");
        }

        if self.current_char().is_digit(10) {
            return self.read_digit();
        }

        if self.current_char().is_alphabetic() {
            return self.read_keyword_or_identifier();
        }

        if self.current_char().is_whitespace() {
            return self.read_whitespace();
        }

        let token = match self.next_char() {
            '+' => Token::new(Kind::PlusToken, "+"),
            '-' => Token::new(Kind::MinusToken, "-"),
            '*' => Token::new(Kind::StarToken, "*"),
            '/' => Token::new(Kind::SlashToken, "/"),
            '%' => Token::new(Kind::ModToken, "%"),
            '(' => Token::new(Kind::OpenParenthesisToken, "("),
            ')' => Token::new(Kind::CloseParenthesisToken, ")"),
            '{' => Token::new(Kind::OpenBracesToken, "{"),
            '}' => Token::new(Kind::CloseBracesToken, "}"),
            ';' => Token::new(Kind::SemicolonToken, ";"),
            ':' => Token::new(Kind::ColonToken, ":"),
            '!' => {
                if self.current_char() == '=' {
                    self.next_char();
                    Token::new(Kind::LogicalNotEquals, "!=")
                } else {
                    Token::new(Kind::LogicalNotToken, "!")
                }
            }
            '~' => Token::new(Kind::BitwiseNotToken, "~"),
            '=' => {
                if self.current_char() == '=' {
                    self.next_char();
                    Token::new(Kind::LogicalEquals, "==")
                } else {
                    Token::new(Kind::EqualsToken, "=")
                }
            }
            '<' => {
                if self.current_char() == '=' {
                    self.next_char();
                    Token::new(Kind::LogicalLessThanOrEquals, "<=")
                } else {
                    Token::new(Kind::LogicalLessThan, "<")
                }
            }
            '>' => {
                if self.current_char() == '=' {
                    self.next_char();
                    Token::new(Kind::LogicalGreaterThanOrEquals, ">=")
                } else {
                    Token::new(Kind::LogicalGreaterThan, ">")
                }
            }
            '&' => {
                if self.current_char() != '&' {
                    Token::new(Kind::BitwiseAndToken, "&")
                } else {
                    self.next_char();
                    Token::new(Kind::LogicalAndToken, "&&")
                }
            }
            '|' => {
                if self.current_char() != '|' {
                    Token::new(Kind::BitwiseOrToken, "|")
                } else {
                    self.next_char();
                    Token::new(Kind::LogicalOrToken, "||")
                }
            }
            '^' => Token::new(Kind::BitwiseXorToken, "^"),
            _ => Token::new(Kind::BadToken, &format!("{}", self.current_char())[..]), // REVIEW: Is that conversion right?
        };

        token
    }

    fn current_char(&self) -> char {
        match self.text.chars().nth(self.current_position) {
            Some(c) => c,
            None => '\0',
        }
    }

    fn next_char(&mut self) -> char {
        let c = self.current_char();
        self.current_position += 1;
        c
    }

    fn read_keyword_or_identifier(&mut self) -> Token {
        let start = self.current_position;

        while self.current_char().is_alphanumeric() && self.current_char() != '\0' {
            self.next_char();
        }

        let end = self.current_position;
        let text = &self.text[start..end];

        match text {
            "true" => Token::new(Kind::TrueToken, "true"),
            "false" => Token::new(Kind::FalseToken, "false"),
            "let" => Token::new(Kind::LetToken, "let"),
            _ => Token::new(Kind::IdentifierToken, text),
        }
    }

    fn read_digit(&mut self) -> Token {
        let start = self.current_position;

        while self.current_char().is_digit(10) && self.current_char() != '\0' {
            self.next_char();
        }

        let end = self.current_position;
        let text = &self.text[start..end];
        Token::new(Kind::NumberToken, text)
    }

    fn read_whitespace(&mut self) -> Token {
        let start = self.current_position;

        while self.current_char().is_whitespace() && self.current_char() != '\0' {
            self.next_char();
        }

        let end = self.current_position;
        let text = &self.text[start..end];
        return Token::new(Kind::WhiteSpaceToken, text);
    }
}
