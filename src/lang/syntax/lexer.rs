#[derive(PartialEq, Eq, Clone, Copy)]
pub enum Kind {
    Identifier,
    Number,
    Boolean,
    OpenParenthesis,
    CloseParenthesis,
    Plus,
    PlusEquals,
    Minus,
    MinusEquals,
    Slash,
    SlashEquals,
    Star,
    StarEquals,
    Mod,
    ModEquals,
    WhiteSpace,
    EndOfFile,
    OpenBraces,
    CloseBraces,
    Semicolon,
    Colon,
    Let,
    Fun,
    Return,
    Bad,
    Equals,
    EqualsEquals,
    Pipe,
    PipePipe,
    PipeEquals,
    Ampersand,
    AmpersandAmpersand,
    AmpersandEquals,
    LessThan,
    LessThanEquals,
    GreaterThan,
    GreaterThanEquals,
    Exclamation,
    ExclamationEquals,
    Circumflex,
    CircumflexEquals,
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
            '+' => {
                if self.current_char() != '=' {
                    Token::new(Kind::Plus, self.position, Some("+"))
                } else {
                    self.next_char();
                    Token::new(Kind::PlusEquals, self.position, Some("+="))
                }
            }
            '-' => {
                if self.current_char() != '=' {
                    Token::new(Kind::Minus, self.position, Some("-"))
                } else {
                    self.next_char();
                    Token::new(Kind::MinusEquals, self.position, Some("-="))
                }
            }
            '*' => {
                if self.current_char() != '=' {
                    Token::new(Kind::Star, self.position, Some("*"))
                } else {
                    self.next_char();
                    Token::new(Kind::StarEquals, self.position, Some("*="))
                }
            }
            '/' => {
                if self.current_char() != '=' {
                    Token::new(Kind::Slash, self.position, Some("/"))
                } else {
                    self.next_char();
                    Token::new(Kind::SlashEquals, self.position, Some("/="))
                }
            }
            '%' => {
                if self.current_char() != '=' {
                    Token::new(Kind::Mod, self.position, Some("%"))
                } else {
                    self.next_char();
                    Token::new(Kind::ModEquals, self.position, Some("%="))
                }
            }
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
                if self.current_char() == '&' {
                    self.next_char();
                    Token::new(Kind::AmpersandAmpersand, self.position, Some("&&"))
                } else if self.current_char() == '=' {
                    self.next_char();
                    Token::new(Kind::AmpersandEquals, self.position, Some("&="))
                } else {
                    Token::new(Kind::Ampersand, self.position, Some("&"))
                }
            }
            '|' => {
                if self.current_char() == '|' {
                    self.next_char();
                    Token::new(Kind::PipePipe, self.position, Some("||"))
                } else if self.current_char() == '=' {
                    self.next_char();
                    Token::new(Kind::PipeEquals, self.position, Some("|="))
                } else {
                    Token::new(Kind::Pipe, self.position, Some("|"))
                }
            }
            '^' => {
                if self.current_char() != '=' {
                    Token::new(Kind::Circumflex, self.position, Some("^"))
                } else {
                    self.next_char();
                    Token::new(Kind::CircumflexEquals, self.position, Some("^="))
                }
            }
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
            "fun" => Token::new(Kind::Fun, self.position, Some("fun")),
            "return" => Token::new(Kind::Return, self.position, Some("return")),
            _ => Token::new(Kind::Identifier, self.position, Some(text)),
        }
    }

    fn read_digit(&mut self) -> Token {
        let start = self.position;

        while self.current_char().is_digit(10) && self.current_char() != '\0' {
            self.next_char();
        }

        if self.current_char() == '.' && self.current_char() != '\0' {
            self.next_char();
        }

        if !self.current_char().is_digit(10) {
            return Token::new(
                Kind::Bad,
                self.position,
                Some(self.current_char().to_string().as_str()),
            );
        }

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
