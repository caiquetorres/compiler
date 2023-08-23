#[derive(PartialEq, Eq, Clone, Copy)]
pub enum Kind {
    Identifier,
    Number,
    Boolean,
    OpenParenthesis,
    CloseParenthesis,
    Plus,
    Minus,
    Slash,
    Star,
    Mod,
    WhiteSpace,
    EndOfFile,
    OpenBraces,
    CloseBraces,
    Semicolon,
    Colon,
    Let,
    Bad,
    Equals,
    EqualsEquals,
    Pipe,
    PipePipe,
    Ampersand,
    AmpersandAmpersand,
    LessThan,
    LessThanEquals,
    GreaterThan,
    GreaterThanEquals,
    Exclamation,
    ExclamationEquals,
    Circumflex,
    Tilde,
}

pub struct Token {
    pub position: usize,
    pub kind: Kind,
    pub value: Option<String>,
}

impl Token {
    pub fn new(kind: Kind, position: usize, value: Option<&str>) -> Self {
        Self {
            kind,
            position,
            value: value.map(|s| s.to_string()),
        }
    }
}

pub struct Lexer {
    position: usize,
    text: String,
}

impl Lexer {
    pub fn new(text: &str) -> Self {
        Self {
            position: 0,
            text: text.to_string(),
        }
    }

    pub fn next(&mut self) -> Token {
        if self.current_char() == '\0' {
            return Token::new(Kind::EndOfFile, self.position, Some("\0"));
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
            '+' => Token::new(Kind::Plus, self.position, Some("+")),
            '-' => Token::new(Kind::Minus, self.position, Some("-")),
            '*' => Token::new(Kind::Star, self.position, Some("*")),
            '/' => Token::new(Kind::Slash, self.position, Some("/")),
            '%' => Token::new(Kind::Mod, self.position, Some("%")),
            '(' => Token::new(Kind::OpenParenthesis, self.position, Some("(")),
            ')' => Token::new(Kind::CloseParenthesis, self.position, Some(")")),
            '{' => Token::new(Kind::OpenBraces, self.position, Some("{")),
            '}' => Token::new(Kind::CloseBraces, self.position, Some("}")),
            ';' => Token::new(Kind::Semicolon, self.position, Some(";")),
            ':' => Token::new(Kind::Colon, self.position, Some(":")),
            '!' => {
                if self.current_char() != '=' {
                    Token::new(Kind::Exclamation, self.position, Some("!"))
                } else {
                    self.next_char();
                    Token::new(Kind::ExclamationEquals, self.position, Some("!="))
                }
            }
            '~' => Token::new(Kind::Tilde, self.position, Some("~")),
            '=' => {
                if self.current_char() == '=' {
                    self.next_char();
                    Token::new(Kind::EqualsEquals, self.position, Some("=="))
                } else {
                    Token::new(Kind::Equals, self.position, Some("="))
                }
            }
            '<' => {
                if self.current_char() != '=' {
                    Token::new(Kind::LessThan, self.position, Some("<"))
                } else {
                    self.next_char();
                    Token::new(Kind::LessThanEquals, self.position, Some("<="))
                }
            }
            '>' => {
                if self.current_char() != '=' {
                    Token::new(Kind::GreaterThan, self.position, Some(">"))
                } else {
                    self.next_char();
                    Token::new(Kind::GreaterThanEquals, self.position, Some(">="))
                }
            }
            '&' => {
                if self.current_char() != '&' {
                    Token::new(Kind::Ampersand, self.position, Some("&"))
                } else {
                    self.next_char();
                    Token::new(Kind::AmpersandAmpersand, self.position, Some("&&"))
                }
            }
            '|' => {
                if self.current_char() != '|' {
                    Token::new(Kind::Pipe, self.position, Some("|"))
                } else {
                    self.next_char();
                    Token::new(Kind::PipePipe, self.position, Some("||"))
                }
            }
            '^' => Token::new(Kind::Circumflex, self.position, Some("^")),
            // TODO: Add &=, |=, ^=, +=, -= *=, /=, %=
            _ => Token::new(
                Kind::Bad,
                self.position,
                Some(self.current_char().to_string().as_str()),
            ),
        };

        token
    }

    fn current_char(&self) -> char {
        match self.text.chars().nth(self.position) {
            Some(c) => c,
            None => '\0',
        }
    }

    fn next_char(&mut self) -> char {
        let c = self.current_char();
        self.position += 1;
        c
    }

    fn read_keyword_or_identifier(&mut self) -> Token {
        let start = self.position;

        while self.current_char().is_alphanumeric() && self.current_char() != '\0' {
            self.next_char();
        }

        let end = self.position;
        let text = &self.text[start..end];

        match text {
            "true" => Token::new(Kind::Boolean, self.position, Some("true")),
            "false" => Token::new(Kind::Boolean, self.position, Some("false")),
            "let" => Token::new(Kind::Let, self.position, Some("let")),
            _ => Token::new(Kind::Identifier, self.position, Some(text)),
        }
    }

    fn read_digit(&mut self) -> Token {
        let start = self.position;

        while self.current_char().is_digit(10) && self.current_char() != '\0' {
            self.next_char();
        }

        let end = self.position;
        let text = &self.text[start..end];
        Token::new(Kind::Number, self.position, Some(text))
    }

    fn read_whitespace(&mut self) -> Token {
        let start = self.position;

        while self.current_char().is_whitespace() && self.current_char() != '\0' {
            self.next_char();
        }

        let end = self.position;
        let text = &self.text[start..end];
        return Token::new(Kind::WhiteSpace, self.position, Some(text));
    }
}
