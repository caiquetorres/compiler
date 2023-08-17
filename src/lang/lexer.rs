use super::{kind::Kind, token::Token};

pub struct Lexer {
    position: usize,
    current_char: char,
    text: String,
}

// REVIEW: We need to refactor this code.
impl Lexer {
    pub fn new(text: &str) -> Self {
        Self {
            position: 0,
            current_char: text.chars().next().unwrap(),
            text: String::from(text),
        }
    }

    pub fn next_token(&mut self) -> Token {
        // REVIEW: Should we convert this code into an iterator?
        if self.position > self.text.len() - 1 {
            return Token::new(Kind::EndOfFileToken, "\0");
        }

        if self.current_char.is_digit(10) {
            let start = self.position as usize;

            while self.current_char.is_digit(10) && self.position < self.text.len() {
                self.next();
            }

            let end = self.position as usize;

            let text = &*self.text;
            return Token::new(Kind::NumberToken, &(*text)[start..end]);
        }

        if self.current_char == ' ' {
            let start = self.position as usize;

            while self.current_char == ' ' && self.position < self.text.len() {
                self.next();
            }

            let end = self.position as usize;

            let text = &self.text[start..end];
            return Token::new(Kind::WhiteSpaceToken, text);
        }

        let token = match self.current_char {
            '+' => Token::new(Kind::PlusToken, "+"),
            '-' => Token::new(Kind::MinusToken, "-"),
            '*' => Token::new(Kind::StarToken, "*"),
            '/' => Token::new(Kind::SlashToken, "/"),
            '(' => Token::new(Kind::OpenParenthesisToken, "("),
            ')' => Token::new(Kind::CloseParenthesisToken, ")"),
            ':' => Token::new(Kind::SemicolonToken, ";"),
            _ => Token::new(Kind::BadToken, ""),
        };

        self.next();

        return token;
    }

    fn next(&mut self) {
        self.position += 1;

        if self.position < self.text.len() {
            self.current_char = self.text.chars().nth(self.position).unwrap();
        }
    }
}
