use super::token::{Position, Token};
use super::token_kind::TokenKind;

pub struct Lexer {
    text: String,
    current_position: Position,
}

impl Lexer {
    pub fn new(text: &str) -> Self {
        Self {
            current_position: Position::new(0, 0, 0),
            text: text.to_string(),
        }
    }

    fn get_current_char(&self) -> char {
        self.text
            .chars()
            .nth(self.current_position.position)
            .unwrap_or('\0')
    }

    fn next_char(&mut self) -> char {
        let current_char = self.get_current_char();
        self.current_position.position += 1;

        if current_char == '\n' {
            self.current_position.column = 0;
            self.current_position.line += 1;
        } else {
            self.current_position.column += 1;
        }

        current_char
    }

    pub fn lex(&mut self) -> Vec<Token> {
        let mut tokens: Vec<Token> = vec![];

        let mut token = self.next();

        while token.kind != TokenKind::EndOfFile {
            if token.kind != TokenKind::WhiteSpace {
                tokens.push(token);
            }
            token = self.next();
        }
        tokens.push(token);

        tokens
    }

    pub fn next(&mut self) -> Token {
        if self.get_current_char() == '\0' {
            return Token::new(TokenKind::EndOfFile, self.current_position, "\0");
        }

        if self.get_current_char().is_digit(10) {
            return self.read_digit();
        }

        if self.get_current_char().is_alphabetic() || self.get_current_char() == '_' {
            return self.read_keyword_or_identifier();
        }

        if self.get_current_char() == '\'' {
            return self.read_char();
        }

        if self.get_current_char() == '"' {
            return self.read_str();
        }

        if self.get_current_char().is_whitespace() {
            return self.read_whitespace();
        }

        let position = self.current_position;

        match self.get_current_char() {
            ';' => {
                self.next_char();
                Token::new(TokenKind::Semicolon, position, ";")
            }
            ',' => {
                self.next_char();
                Token::new(TokenKind::Comma, position, ",")
            }
            ':' => {
                self.next_char();
                Token::new(TokenKind::Colon, position, ":")
            }
            '{' => {
                self.next_char();
                Token::new(TokenKind::OpenBraces, position, "{")
            }
            '}' => {
                self.next_char();
                Token::new(TokenKind::CloseBraces, position, "}")
            }
            '(' => {
                self.next_char();
                Token::new(TokenKind::OpenParenthesis, position, "(")
            }
            ')' => {
                self.next_char();
                Token::new(TokenKind::CloseParenthesis, position, ")")
            }
            '[' => {
                self.next_char();
                Token::new(TokenKind::OpenBrackets, position, "[")
            }
            ']' => {
                self.next_char();
                Token::new(TokenKind::CloseBrackets, position, "]")
            }
            '.' => {
                self.next_char();
                match self.get_current_char() {
                    '.' => {
                        self.next_char();
                        match self.get_current_char() {
                            '=' => {
                                self.next_char();
                                Token::new(TokenKind::DotDotEquals, position, "..=")
                            }
                            _ => Token::new(TokenKind::DotDot, position, ".."),
                        }
                    }
                    _ => Token::new(TokenKind::Dot, position, "."),
                }
            }
            '<' => {
                self.next_char();
                match self.get_current_char() {
                    '=' => {
                        self.next_char();
                        Token::new(TokenKind::LessThanEquals, position, "<=")
                    }
                    '<' => {
                        self.next_char();
                        Token::new(TokenKind::LessThanLessThan, position, "<<")
                    }
                    _ => Token::new(TokenKind::LessThan, position, "<"),
                }
            }
            '>' => {
                self.next_char();
                match self.get_current_char() {
                    '=' => {
                        self.next_char();
                        Token::new(TokenKind::GreaterThanEquals, position, ">=")
                    }
                    '>' => {
                        self.next_char();
                        Token::new(TokenKind::GreaterThanGreaterThan, position, ">>")
                    }
                    _ => Token::new(TokenKind::GreaterThan, position, ">"),
                }
            }
            '=' => {
                self.next_char();
                match self.get_current_char() {
                    '=' => {
                        self.next_char();
                        Token::new(TokenKind::EqualsEquals, position, "==")
                    }
                    _ => Token::new(TokenKind::Equals, position, "="),
                }
            }
            '!' => {
                self.next_char();
                match self.get_current_char() {
                    '=' => {
                        self.next_char();
                        Token::new(TokenKind::ExclamationEquals, position, "!=")
                    }
                    _ => Token::new(TokenKind::Exclamation, position, "!"),
                }
            }
            '&' => {
                self.next_char();
                match self.get_current_char() {
                    '=' => {
                        self.next_char();
                        Token::new(TokenKind::AmpersandEquals, position, "&=")
                    }
                    '&' => {
                        self.next_char();
                        Token::new(TokenKind::AmpersandAmpersand, position, "&&")
                    }
                    _ => Token::new(TokenKind::Ampersand, position, "&"),
                }
            }
            '|' => {
                self.next_char();
                match self.get_current_char() {
                    '=' => {
                        self.next_char();
                        Token::new(TokenKind::PipeEquals, position, "|=")
                    }
                    '|' => {
                        self.next_char();
                        Token::new(TokenKind::PipePipe, position, "||")
                    }
                    _ => Token::new(TokenKind::Pipe, position, "|"),
                }
            }
            '~' => {
                self.next_char();
                match self.get_current_char() {
                    '=' => {
                        self.next_char();
                        Token::new(TokenKind::TildeEquals, position, "~=")
                    }
                    _ => Token::new(TokenKind::Tilde, position, "~"),
                }
            }
            '^' => {
                self.next_char();
                match self.get_current_char() {
                    '=' => {
                        self.next_char();
                        Token::new(TokenKind::CircumflexEquals, position, "^=")
                    }
                    _ => Token::new(TokenKind::Circumflex, position, "^"),
                }
            }
            '+' => {
                self.next_char();
                match self.get_current_char() {
                    '=' => {
                        self.next_char();
                        Token::new(TokenKind::PlusEquals, position, "+=")
                    }
                    _ => Token::new(TokenKind::Plus, position, "+"),
                }
            }
            '-' => {
                self.next_char();
                match self.get_current_char() {
                    '=' => {
                        self.next_char();
                        Token::new(TokenKind::MinusEquals, position, "-=")
                    }
                    _ => Token::new(TokenKind::Minus, position, "-"),
                }
            }
            '*' => {
                self.next_char();
                match self.get_current_char() {
                    '=' => {
                        self.next_char();
                        Token::new(TokenKind::StarEquals, position, "*=")
                    }
                    _ => Token::new(TokenKind::Star, position, "*"),
                }
            }
            '/' => {
                self.next_char();
                match self.get_current_char() {
                    '=' => {
                        self.next_char();
                        Token::new(TokenKind::SlashEquals, position, "/=")
                    }
                    '/' => {
                        self.next_char();
                        self.read_single_line_comment();
                        self.next()
                    }
                    '*' => {
                        self.next_char();
                        self.read_multi_line_comment();
                        self.next()
                    }
                    _ => Token::new(TokenKind::Slash, position, "/"),
                }
            }
            '%' => {
                self.next_char();
                match self.get_current_char() {
                    '=' => {
                        self.next_char();
                        Token::new(TokenKind::ModEquals, position, "%=")
                    }
                    _ => Token::new(TokenKind::Mod, position, "%"),
                }
            }
            _ => Token::new(TokenKind::Bad, position, ""),
        }
    }

    fn read_digit(&mut self) -> Token {
        let position = self.current_position;
        let start = self.current_position.position;
        let mut end = self.current_position.position;
        let number: &str;

        while self.get_current_char().is_digit(10) {
            end += 1;
            self.next_char();
        }

        // decimal value
        if self.get_current_char() != '.' {
            number = &self.text[start..end];
            return Token::new(TokenKind::Number, position, number);
        }

        if let Some(c) = self.text.chars().nth(self.current_position.position + 1) {
            if c == '.' {
                number = &self.text[start..end];
                return Token::new(TokenKind::Number, position, number);
            }
        }

        end += 1;
        self.next_char();

        if !self.get_current_char().is_digit(10) {
            return Token::new(TokenKind::Bad, self.current_position, "");
        }

        while self.get_current_char().is_digit(10) {
            end += 1;
            self.next_char();
        }

        number = &self.text[start..end];
        Token::new(TokenKind::Number, position, number)
    }

    fn read_char(&mut self) -> Token {
        let position = self.current_position;
        let mut start = self.current_position.position;
        let mut end = self.current_position.position;

        // consumes the "'"
        end += 1;
        start += 1;
        self.next_char();

        if self.get_current_char() == '\\' {
            end += 1;

            // consumes the "\"
            self.next_char();
        }

        if self.get_current_char().is_alphanumeric() {
            end += 1;

            // consumes the char
            self.next_char();
        }

        if self.get_current_char() == '\'' {
            // consumes the "'"
            self.next_char();

            let c = &self.text[start..end];
            return Token::new(TokenKind::Char, position, c);
        }

        Token::new(TokenKind::Bad, self.current_position, "")
    }

    fn read_str(&mut self) -> Token {
        let position = self.current_position;
        let mut start = self.current_position.position;
        let mut end = self.current_position.position;

        // consumes the '"'
        end += 1;
        start += 1;
        self.next_char();

        while self.get_current_char() != '"' && self.get_current_char() != '\0' {
            end += 1;
            self.next_char();
        }

        if self.get_current_char() != '"' {
            return Token::new(TokenKind::Bad, self.current_position, "");
        }

        // consumes the '"'
        self.next_char();

        let text = &self.text[start..end];
        Token::new(TokenKind::String, position, text)
    }

    fn read_single_line_comment(&mut self) {
        while self.get_current_char() != '\n' {
            self.next_char();
        }
        self.next_char();
    }

    fn read_multi_line_comment(&mut self) {
        loop {
            let pos = self.current_position.position;
            let current_char = self.text.chars().nth(pos).unwrap_or(' ');
            let next_char = self.text.chars().nth(pos + 1).unwrap_or(' ');

            if current_char == '*' && next_char == '/' {
                self.next_char();
                self.next_char();
                break;
            }

            self.next();
        }
    }

    fn read_keyword_or_identifier(&mut self) -> Token {
        let position = self.current_position;
        let start = self.current_position.position;
        let mut end = self.current_position.position;

        while self.get_current_char().is_alphanumeric() || self.get_current_char() == '_' {
            self.next_char();
            end += 1;
        }

        let id = &self.text[start..end];

        match id {
            "fun" => Token::new(TokenKind::Fun, position, "fun"),
            "let" => Token::new(TokenKind::Let, position, "let"),
            "const" => Token::new(TokenKind::Const, position, "const"),
            "return" => Token::new(TokenKind::Return, position, "return"),
            "while" => Token::new(TokenKind::While, position, "while"),
            "do" => Token::new(TokenKind::Do, position, "do"),
            "for" => Token::new(TokenKind::For, position, "for"),
            "in" => Token::new(TokenKind::In, position, "in"),
            "if" => Token::new(TokenKind::If, position, "if"),
            "else" => Token::new(TokenKind::Else, position, "else"),
            "true" => Token::new(TokenKind::Boolean, position, "true"),
            "false" => Token::new(TokenKind::Boolean, position, "false"),
            _ => Token::new(TokenKind::Identifier, position, id),
        }
    }

    fn read_whitespace(&mut self) -> Token {
        let position = self.current_position;

        while self.get_current_char().is_whitespace() && self.get_current_char() != '\0' {
            self.next_char();
        }

        return Token::new(TokenKind::WhiteSpace, position, "");
    }
}

#[cfg(test)]
mod tests {
    use crate::lang::syntax::lexer::{
        lexer::{Lexer, TokenKind},
        token::Token,
    };

    #[test]
    fn test_comment() {
        let code = "/* Comment */
        fun main() {

        }
        ";
        let mut lexer = Lexer::new(code);

        lexer.next();
        assert_eq!(lexer.next().kind, TokenKind::Fun);

        let code = "
        fun main() {
            // test
        }
        ";
        let mut lexer = Lexer::new(code);

        lexer.next();
        assert_eq!(lexer.next().kind, TokenKind::Fun);
    }

    #[test]
    fn test_range_operator() {
        let code = "..";
        let mut lexer = Lexer::new(code);

        assert_eq!(lexer.next().kind, TokenKind::DotDot);

        let code = "2..3";
        let mut lexer = Lexer::new(code);

        assert_eq!(lexer.next().kind, TokenKind::Number);
        assert_eq!(lexer.next().kind, TokenKind::DotDot);
        assert_eq!(lexer.next().kind, TokenKind::Number);

        let code = "2..=3";
        let mut lexer = Lexer::new(code);

        assert_eq!(lexer.next().kind, TokenKind::Number);
        assert_eq!(lexer.next().kind, TokenKind::DotDotEquals);
        assert_eq!(lexer.next().kind, TokenKind::Number);

        let code = "2..";
        let mut lexer = Lexer::new(code);

        assert_eq!(lexer.next().kind, TokenKind::Number);
        assert_eq!(lexer.next().kind, TokenKind::DotDot);

        let code = "..3";
        let mut lexer = Lexer::new(code);

        assert_eq!(lexer.next().kind, TokenKind::DotDot);
        assert_eq!(lexer.next().kind, TokenKind::Number);

        let code = "..=3";
        let mut lexer = Lexer::new(code);

        assert_eq!(lexer.next().kind, TokenKind::DotDotEquals);
        assert_eq!(lexer.next().kind, TokenKind::Number);
    }

    #[test]
    fn test_brackets_braces_parenthesis_token() {
        let code = "([{}])";
        let mut lexer = Lexer::new(code);

        assert_eq!(lexer.next().kind, TokenKind::OpenParenthesis);
        assert_eq!(lexer.next().kind, TokenKind::OpenBrackets);
        assert_eq!(lexer.next().kind, TokenKind::OpenBraces);
        assert_eq!(lexer.next().kind, TokenKind::CloseBraces);
        assert_eq!(lexer.next().kind, TokenKind::CloseBrackets);
        assert_eq!(lexer.next().kind, TokenKind::CloseParenthesis);
    }

    #[test]
    fn test_plus_token() {
        let code = "+ +=";
        let mut token: Token;
        let mut lexer = Lexer::new(code);

        token = lexer.next();
        assert_eq!(token.kind, TokenKind::Plus);

        // white space
        lexer.next();

        token = lexer.next();
        assert_eq!(token.kind, TokenKind::PlusEquals);
    }

    #[test]
    fn test_minus_token() {
        let code = "- -=";
        let mut token: Token;
        let mut lexer = Lexer::new(code);

        token = lexer.next();
        assert_eq!(token.kind, TokenKind::Minus);

        // white space
        lexer.next();

        token = lexer.next();
        assert_eq!(token.kind, TokenKind::MinusEquals);
    }

    #[test]
    fn test_star_token() {
        let code = "* *=";
        let mut token: Token;
        let mut lexer = Lexer::new(code);

        token = lexer.next();
        assert_eq!(token.kind, TokenKind::Star);

        // white space
        lexer.next();

        token = lexer.next();
        assert_eq!(token.kind, TokenKind::StarEquals);
    }

    #[test]
    fn test_slash_token() {
        let code = "/ /=";
        let mut token: Token;
        let mut lexer = Lexer::new(code);

        token = lexer.next();
        assert_eq!(token.kind, TokenKind::Slash);

        // white space
        lexer.next();

        token = lexer.next();
        assert_eq!(token.kind, TokenKind::SlashEquals);
    }

    #[test]
    fn test_mod_token() {
        let code = "% %=";
        let mut token: Token;
        let mut lexer = Lexer::new(code);

        token = lexer.next();
        assert_eq!(token.kind, TokenKind::Mod);

        // white space
        lexer.next();

        token = lexer.next();
        assert_eq!(token.kind, TokenKind::ModEquals);
    }

    #[test]
    fn test_ampersand_token() {
        let code = "& && &=";
        let mut token: Token;
        let mut lexer = Lexer::new(code);

        token = lexer.next();
        assert_eq!(token.kind, TokenKind::Ampersand);

        // white space
        lexer.next();

        token = lexer.next();
        assert_eq!(token.kind, TokenKind::AmpersandAmpersand);

        // white space
        lexer.next();

        token = lexer.next();
        assert_eq!(token.kind, TokenKind::AmpersandEquals);
    }

    #[test]
    fn test_pipe_token() {
        let code = "| || |=";
        let mut token: Token;
        let mut lexer = Lexer::new(code);

        token = lexer.next();
        assert_eq!(token.kind, TokenKind::Pipe);

        // white space
        lexer.next();

        token = lexer.next();
        assert_eq!(token.kind, TokenKind::PipePipe);

        // white space
        lexer.next();

        token = lexer.next();
        assert_eq!(token.kind, TokenKind::PipeEquals);
    }

    #[test]
    fn test_exclamation_token() {
        let code = "! !=";
        let mut token: Token;
        let mut lexer = Lexer::new(code);

        token = lexer.next();
        assert_eq!(token.kind, TokenKind::Exclamation);

        // white space
        lexer.next();

        token = lexer.next();
        assert_eq!(token.kind, TokenKind::ExclamationEquals);
    }

    #[test]
    fn test_equals_token() {
        let code = "= ==";
        let mut token: Token;
        let mut lexer = Lexer::new(code);

        token = lexer.next();
        assert_eq!(token.kind, TokenKind::Equals);

        // white space
        lexer.next();

        token = lexer.next();
        assert_eq!(token.kind, TokenKind::EqualsEquals);
    }

    #[test]
    fn test_greater_than_token() {
        let code = "> >= >>";
        let mut token: Token;
        let mut lexer = Lexer::new(code);

        token = lexer.next();
        assert_eq!(token.kind, TokenKind::GreaterThan);

        // white space
        lexer.next();

        token = lexer.next();
        assert_eq!(token.kind, TokenKind::GreaterThanEquals);

        // white space
        lexer.next();

        token = lexer.next();
        assert_eq!(token.kind, TokenKind::GreaterThanGreaterThan);
    }

    #[test]
    fn test_less_than_token() {
        let code = "< <= <<";
        let mut token: Token;
        let mut lexer = Lexer::new(code);

        token = lexer.next();
        assert_eq!(token.kind, TokenKind::LessThan);

        // white space
        lexer.next();

        token = lexer.next();
        assert_eq!(token.kind, TokenKind::LessThanEquals);

        // white space
        lexer.next();

        token = lexer.next();
        assert_eq!(token.kind, TokenKind::LessThanLessThan);
    }

    #[test]
    fn test_tilde_token() {
        let code = "~ ~=";
        let mut token: Token;
        let mut lexer = Lexer::new(code);

        token = lexer.next();
        assert_eq!(token.kind, TokenKind::Tilde);

        // white space
        lexer.next();

        token = lexer.next();
        assert_eq!(token.kind, TokenKind::TildeEquals);
    }

    #[test]
    fn test_circumflex_token() {
        let code = "^ ^=";
        let mut token: Token;
        let mut lexer = Lexer::new(code);

        token = lexer.next();
        assert_eq!(token.kind, TokenKind::Circumflex);

        // white space
        lexer.next();

        token = lexer.next();
        assert_eq!(token.kind, TokenKind::CircumflexEquals);
    }

    #[test]
    fn test_identifier_token() {
        let code = "while true for variable";
        let mut token: Token;
        let mut lexer = Lexer::new(code);

        token = lexer.next();
        assert_eq!(token.kind, TokenKind::While);

        // white space
        lexer.next();

        token = lexer.next();
        assert_eq!(token.kind, TokenKind::Boolean);

        // white space
        lexer.next();

        token = lexer.next();
        assert_eq!(token.kind, TokenKind::For);

        // white space
        lexer.next();

        token = lexer.next();
        assert_eq!(token.kind, TokenKind::Identifier);
    }

    #[test]
    fn test_number_token() {
        let mut code = "23";
        let mut token: Token;
        let mut lexer = Lexer::new(code);

        token = lexer.next();
        assert_eq!(token.kind, TokenKind::Number);
        assert_eq!(token.value, "23");

        code = "23.2";
        lexer = Lexer::new(code);

        token = lexer.next();
        assert_eq!(token.kind, TokenKind::Number);
        assert_eq!(token.value, "23.2");

        code = "2.";
        lexer = Lexer::new(code);

        token = lexer.next();
        assert_eq!(token.kind, TokenKind::Bad);
    }

    #[test]
    fn test_char_token() {
        let mut code = "'c'";
        let mut token: Token;
        let mut lexer = Lexer::new(code);

        token = lexer.next();
        assert_eq!(token.kind, TokenKind::Char);
        assert_eq!(token.value, "c");

        code = "'\\0'";
        lexer = Lexer::new(code);

        token = lexer.next();
        assert_eq!(token.kind, TokenKind::Char);

        code = "'c";
        lexer = Lexer::new(code);
        token = lexer.next();

        assert_eq!(token.kind, TokenKind::Bad);

        code = "\\";
        lexer = Lexer::new(code);
        token = lexer.next();

        assert_eq!(token.kind, TokenKind::Bad);
    }

    #[test]
    fn test_string_token() {
        let mut code = "\"test string\"";
        let mut token: Token;
        let mut lexer = Lexer::new(code);

        token = lexer.next();
        assert_eq!(token.kind, TokenKind::String);
        assert_eq!(token.value, "test string");

        code = "\"test string 2\"";
        lexer = Lexer::new(code);

        token = lexer.next();
        assert_eq!(token.kind, TokenKind::String);
        assert_eq!(token.value, "test string 2");

        code = "'\"test string";
        lexer = Lexer::new(code);

        token = lexer.next();
        assert_eq!(token.kind, TokenKind::Bad);
    }
}
