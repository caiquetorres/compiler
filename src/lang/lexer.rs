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
        if self.current_position > self.text.len() - 1 {
            return Token::new(Kind::EndOfFileToken, "");
        }

        if self.current_char().is_digit(10) {
            let start = self.current_position;

            while self.current_char().is_digit(10) && self.current_position < self.text.len() {
                self.next_char();
            }

            let end = self.current_position;
            return Token::new(Kind::NumberToken, &self.text[start..end]);
        }

        if self.current_char().is_alphabetic() {
            let start = self.current_position;

            while self.current_char().is_alphabetic() {
                self.next_char();
            }

            let end = self.current_position;
            let word = &self.text[start..end];

            // TODO: Add more keywords or create some logic for validating them.

            if word == "true" {
                return Token::new(Kind::TrueToken, word);
            }

            return Token::new(Kind::FalseToken, word);
        }

        if self.current_char().is_whitespace() {
            let start = self.current_position;

            while self.current_char().is_whitespace() && self.current_position < self.text.len() {
                self.next_char();
            }

            let end = self.current_position;
            return Token::new(Kind::WhiteSpaceToken, &self.text[start..end]);
        }

        let token = match self.current_char() {
            '+' => Token::new(Kind::PlusToken, "+"),
            '-' => Token::new(Kind::MinusToken, "-"),
            '*' => Token::new(Kind::StarToken, "*"),
            '/' => Token::new(Kind::SlashToken, "/"),
            '%' => Token::new(Kind::ModToken, "%"),
            '(' => Token::new(Kind::OpenParenthesisToken, "("),
            ')' => Token::new(Kind::CloseParenthesisToken, ")"),
            ':' => Token::new(Kind::SemicolonToken, ";"),
            _ => Token::new(Kind::BadToken, &format!("{}", self.current_char())[..]), // REVIEW: Is that conversion right?
        };

        self.next_char();

        token
    }

    fn current_char(&self) -> char {
        match self.text.chars().nth(self.current_position) {
            Some(c) => c,
            None => '\0',
        }
    }

    fn next_char(&mut self) {
        self.current_position += 1;
        match self.text.chars().nth(self.current_position) {
            Some(_) => (),
            None => (),
        }
    }
}
