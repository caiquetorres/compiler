use super::kind::Kind;
use super::token::{Position, Token};

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

    fn current_char(&self) -> char {
        self.text
            .chars()
            .nth(self.current_position.position)
            .unwrap_or('\0')
    }

    fn next_char(&mut self) -> char {
        let current_char = self.current_char();
        self.current_position.position += 1;

        if current_char == '\n' {
            self.current_position.column = 0;
            self.current_position.line += 1;
        } else {
            self.current_position.column += 1;
        }

        current_char
    }

    pub fn next(&mut self) -> Token {
        if self.current_char() == '\0' {
            return Token::new(Kind::EndOfFile, self.current_position, "\0");
        }

        if self.current_char().is_digit(10) {
            return self.read_digit();
        }

        if self.current_char().is_alphabetic() || self.current_char() == '_' {
            return self.read_keyword_or_identifier();
        }

        if self.current_char() == '\'' {
            return self.read_char();
        }

        if self.current_char() == '"' {
            return self.read_str();
        }

        if self.current_char().is_whitespace() {
            return self.read_whitespace();
        }

        let position = self.current_position;

        match self.current_char() {
            ';' => {
                self.next_char();
                Token::new(Kind::Semicolon, position, ";")
            }
            ',' => {
                self.next_char();
                Token::new(Kind::Comma, position, ",")
            }
            ':' => {
                self.next_char();
                Token::new(Kind::Colon, position, ":")
            }
            '{' => {
                self.next_char();
                Token::new(Kind::OpenBraces, position, "{")
            }
            '}' => {
                self.next_char();
                Token::new(Kind::CloseBraces, position, "}")
            }
            '(' => {
                self.next_char();
                Token::new(Kind::OpenParenthesis, position, "(")
            }
            ')' => {
                self.next_char();
                Token::new(Kind::CloseParenthesis, position, ")")
            }
            '[' => {
                self.next_char();
                Token::new(Kind::OpenBrackets, position, "[")
            }
            ']' => {
                self.next_char();
                Token::new(Kind::CloseBrackets, position, "]")
            }
            '.' => {
                self.next_char();
                match self.current_char() {
                    '.' => {
                        self.next_char();
                        match self.current_char() {
                            '=' => {
                                self.next_char();
                                Token::new(Kind::DotDotEquals, position, "..=")
                            }
                            _ => Token::new(Kind::DotDot, position, ".."),
                        }
                    }
                    _ => Token::new(Kind::Dot, position, "."),
                }
            }
            '<' => {
                self.next_char();
                match self.current_char() {
                    '=' => {
                        self.next_char();
                        Token::new(Kind::LessThanEquals, position, "<=")
                    }
                    '<' => {
                        self.next_char();
                        Token::new(Kind::LessThanLessThan, position, "<<")
                    }
                    _ => Token::new(Kind::LessThan, position, "<"),
                }
            }
            '>' => {
                self.next_char();
                match self.current_char() {
                    '=' => {
                        self.next_char();
                        Token::new(Kind::GreaterThanEquals, position, ">=")
                    }
                    '>' => {
                        self.next_char();
                        Token::new(Kind::GreaterThanGreaterThan, position, ">>")
                    }
                    _ => Token::new(Kind::GreaterThan, position, ">"),
                }
            }
            '=' => {
                self.next_char();
                match self.current_char() {
                    '=' => {
                        self.next_char();
                        Token::new(Kind::EqualsEquals, position, "==")
                    }
                    _ => Token::new(Kind::Equals, position, "="),
                }
            }
            '!' => {
                self.next_char();
                match self.current_char() {
                    '=' => {
                        self.next_char();
                        Token::new(Kind::ExclamationEquals, position, "!=")
                    }
                    _ => Token::new(Kind::Exclamation, position, "!"),
                }
            }
            '&' => {
                self.next_char();
                match self.current_char() {
                    '=' => {
                        self.next_char();
                        Token::new(Kind::AmpersandEquals, position, "&=")
                    }
                    '&' => {
                        self.next_char();
                        Token::new(Kind::AmpersandAmpersand, position, "&&")
                    }
                    _ => Token::new(Kind::Ampersand, position, "&"),
                }
            }
            '|' => {
                self.next_char();
                match self.current_char() {
                    '=' => {
                        self.next_char();
                        Token::new(Kind::PipeEquals, position, "|=")
                    }
                    '|' => {
                        self.next_char();
                        Token::new(Kind::PipePipe, position, "||")
                    }
                    _ => Token::new(Kind::Pipe, position, "|"),
                }
            }
            '~' => {
                self.next_char();
                match self.current_char() {
                    '=' => {
                        self.next_char();
                        Token::new(Kind::TildeEquals, position, "~=")
                    }
                    _ => Token::new(Kind::Tilde, position, "~"),
                }
            }
            '^' => {
                self.next_char();
                match self.current_char() {
                    '=' => {
                        self.next_char();
                        Token::new(Kind::CircumflexEquals, position, "^=")
                    }
                    _ => Token::new(Kind::Circumflex, position, "^"),
                }
            }
            '+' => {
                self.next_char();
                match self.current_char() {
                    '=' => {
                        self.next_char();
                        Token::new(Kind::PlusEquals, position, "+=")
                    }
                    '+' => {
                        self.next_char();
                        Token::new(Kind::PlusPlus, position, "++")
                    }
                    _ => Token::new(Kind::Plus, position, "+"),
                }
            }
            '-' => {
                self.next_char();
                match self.current_char() {
                    '=' => {
                        self.next_char();
                        Token::new(Kind::MinusEquals, position, "-=")
                    }
                    '-' => {
                        self.next_char();
                        Token::new(Kind::MinusMinus, position, "-")
                    }
                    _ => Token::new(Kind::Minus, position, "-"),
                }
            }
            '*' => {
                self.next_char();
                match self.current_char() {
                    '=' => {
                        self.next_char();
                        Token::new(Kind::StarEquals, position, "*=")
                    }
                    _ => Token::new(Kind::Star, position, "*"),
                }
            }
            '/' => {
                self.next_char();
                match self.current_char() {
                    '=' => {
                        self.next_char();
                        Token::new(Kind::SlashEquals, position, "/=")
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
                    _ => Token::new(Kind::Slash, position, "/"),
                }
            }
            '%' => {
                self.next_char();
                match self.current_char() {
                    '=' => {
                        self.next_char();
                        Token::new(Kind::ModEquals, position, "%=")
                    }
                    _ => Token::new(Kind::Mod, position, "%"),
                }
            }
            _ => Token::new(Kind::Bad, position, ""),
        }
    }

    fn read_digit(&mut self) -> Token {
        let position = self.current_position;
        let start = self.current_position.position;
        let mut end = self.current_position.position;
        let number: &str;

        while self.current_char().is_digit(10) {
            end += 1;
            self.next_char();
        }

        // decimal value
        if self.current_char() != '.' {
            number = &self.text[start..end];
            return Token::new(Kind::Number, position, number);
        }

        if let Some(c) = self.text.chars().nth(self.current_position.position + 1) {
            if c == '.' {
                number = &self.text[start..end];
                return Token::new(Kind::Number, position, number);
            }
        }

        end += 1;
        self.next_char();

        if !self.current_char().is_digit(10) {
            return Token::new(Kind::Bad, self.current_position, "");
        }

        while self.current_char().is_digit(10) {
            end += 1;
            self.next_char();
        }

        number = &self.text[start..end];
        Token::new(Kind::Number, position, number)
    }

    fn read_char(&mut self) -> Token {
        let position = self.current_position;
        let mut start = self.current_position.position;
        let mut end = self.current_position.position;

        // consumes the "'"
        end += 1;
        start += 1;
        self.next_char();

        if self.current_char() == '\\' {
            end += 1;

            // consumes the "\"
            self.next_char();
        }

        if self.current_char().is_alphanumeric() {
            end += 1;

            // consumes the char
            self.next_char();
        }

        if self.current_char() == '\'' {
            // consumes the "'"
            self.next_char();

            let c = &self.text[start..end];
            return Token::new(Kind::Char, position, c);
        }

        Token::new(Kind::Bad, self.current_position, "")
    }

    fn read_str(&mut self) -> Token {
        let position = self.current_position;
        let mut start = self.current_position.position;
        let mut end = self.current_position.position;

        // consumes the '"'
        end += 1;
        start += 1;
        self.next_char();

        while self.current_char() != '"' && self.current_char() != '\0' {
            end += 1;
            self.next_char();
        }

        if self.current_char() != '"' {
            return Token::new(Kind::Bad, self.current_position, "");
        }

        // consumes the '"'
        self.next_char();

        let text = &self.text[start..end];
        Token::new(Kind::String, position, text)
    }

    fn read_single_line_comment(&mut self) {
        while self.current_char() != '\n' {
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

        while self.current_char().is_alphanumeric() || self.current_char() == '_' {
            self.next_char();
            end += 1;
        }

        let id = &self.text[start..end];

        match id {
            "fun" => Token::new(Kind::Fun, position, "fun"),
            "let" => Token::new(Kind::Let, position, "let"),
            "const" => Token::new(Kind::Const, position, "const"),
            "return" => Token::new(Kind::Return, position, "return"),
            "while" => Token::new(Kind::While, position, "while"),
            "do" => Token::new(Kind::Do, position, "do"),
            "for" => Token::new(Kind::For, position, "for"),
            "in" => Token::new(Kind::In, position, "in"),
            "if" => Token::new(Kind::If, position, "if"),
            "else" => Token::new(Kind::Else, position, "else"),
            "true" => Token::new(Kind::Boolean, position, "true"),
            "false" => Token::new(Kind::Boolean, position, "false"),
            _ => Token::new(Kind::Identifier, position, id),
        }
    }

    fn read_whitespace(&mut self) -> Token {
        let position = self.current_position;

        while self.current_char().is_whitespace() && self.current_char() != '\0' {
            self.next_char();
        }

        return Token::new(Kind::WhiteSpace, position, "");
    }
}

#[cfg(test)]
mod tests {
    use crate::lang::syntax::lexer::{
        lexer::{Kind, Lexer},
        token::Token,
    };

    #[test]
    fn test_comment() {
        let code = "/* Comment */
        fun main() { }
        ";
        let mut lexer = Lexer::new(code);

        lexer.next();
        assert_eq!(lexer.next().kind, Kind::Fun);

        let code = "
        fun main() {
            //test
         }
        ";
        let mut lexer = Lexer::new(code);

        lexer.next();
        assert_eq!(lexer.next().kind, Kind::Fun);
    }

    #[test]
    fn test_range_operator() {
        let code = "..";
        let mut lexer = Lexer::new(code);

        assert_eq!(lexer.next().kind, Kind::DotDot);

        let code = "2..3";
        let mut lexer = Lexer::new(code);

        assert_eq!(lexer.next().kind, Kind::Number);
        assert_eq!(lexer.next().kind, Kind::DotDot);
        assert_eq!(lexer.next().kind, Kind::Number);

        let code = "2..=3";
        let mut lexer = Lexer::new(code);

        assert_eq!(lexer.next().kind, Kind::Number);
        assert_eq!(lexer.next().kind, Kind::DotDotEquals);
        assert_eq!(lexer.next().kind, Kind::Number);

        let code = "2..";
        let mut lexer = Lexer::new(code);

        assert_eq!(lexer.next().kind, Kind::Number);
        assert_eq!(lexer.next().kind, Kind::DotDot);

        let code = "..3";
        let mut lexer = Lexer::new(code);

        assert_eq!(lexer.next().kind, Kind::DotDot);
        assert_eq!(lexer.next().kind, Kind::Number);

        let code = "..=3";
        let mut lexer = Lexer::new(code);

        assert_eq!(lexer.next().kind, Kind::DotDotEquals);
        assert_eq!(lexer.next().kind, Kind::Number);
    }

    #[test]
    fn test_brackets_braces_parenthesis_token() {
        let code = "([{}])";
        let mut lexer = Lexer::new(code);

        assert_eq!(lexer.next().kind, Kind::OpenParenthesis);
        assert_eq!(lexer.next().kind, Kind::OpenBrackets);
        assert_eq!(lexer.next().kind, Kind::OpenBraces);
        assert_eq!(lexer.next().kind, Kind::CloseBraces);
        assert_eq!(lexer.next().kind, Kind::CloseBrackets);
        assert_eq!(lexer.next().kind, Kind::CloseParenthesis);
    }

    #[test]
    fn test_plus_token() {
        let code = "+ ++ +=";
        let mut token: Token;
        let mut lexer = Lexer::new(code);

        token = lexer.next();
        assert_eq!(token.kind, Kind::Plus);

        // white space
        lexer.next();

        token = lexer.next();
        assert_eq!(token.kind, Kind::PlusPlus);

        // white space
        lexer.next();

        token = lexer.next();
        assert_eq!(token.kind, Kind::PlusEquals);
    }

    #[test]
    fn test_minus_token() {
        let code = "- -- -=";
        let mut token: Token;
        let mut lexer = Lexer::new(code);

        token = lexer.next();
        assert_eq!(token.kind, Kind::Minus);

        // white space
        lexer.next();

        token = lexer.next();
        assert_eq!(token.kind, Kind::MinusMinus);

        // white space
        lexer.next();

        token = lexer.next();
        assert_eq!(token.kind, Kind::MinusEquals);
    }

    #[test]
    fn test_star_token() {
        let code = "* *=";
        let mut token: Token;
        let mut lexer = Lexer::new(code);

        token = lexer.next();
        assert_eq!(token.kind, Kind::Star);

        // white space
        lexer.next();

        token = lexer.next();
        assert_eq!(token.kind, Kind::StarEquals);
    }

    #[test]
    fn test_slash_token() {
        let code = "/ /=";
        let mut token: Token;
        let mut lexer = Lexer::new(code);

        token = lexer.next();
        assert_eq!(token.kind, Kind::Slash);

        // white space
        lexer.next();

        token = lexer.next();
        assert_eq!(token.kind, Kind::SlashEquals);
    }

    #[test]
    fn test_mod_token() {
        let code = "% %=";
        let mut token: Token;
        let mut lexer = Lexer::new(code);

        token = lexer.next();
        assert_eq!(token.kind, Kind::Mod);

        // white space
        lexer.next();

        token = lexer.next();
        assert_eq!(token.kind, Kind::ModEquals);
    }

    #[test]
    fn test_ampersand_token() {
        let code = "& && &=";
        let mut token: Token;
        let mut lexer = Lexer::new(code);

        token = lexer.next();
        assert_eq!(token.kind, Kind::Ampersand);

        // white space
        lexer.next();

        token = lexer.next();
        assert_eq!(token.kind, Kind::AmpersandAmpersand);

        // white space
        lexer.next();

        token = lexer.next();
        assert_eq!(token.kind, Kind::AmpersandEquals);
    }

    #[test]
    fn test_pipe_token() {
        let code = "| || |=";
        let mut token: Token;
        let mut lexer = Lexer::new(code);

        token = lexer.next();
        assert_eq!(token.kind, Kind::Pipe);

        // white space
        lexer.next();

        token = lexer.next();
        assert_eq!(token.kind, Kind::PipePipe);

        // white space
        lexer.next();

        token = lexer.next();
        assert_eq!(token.kind, Kind::PipeEquals);
    }

    #[test]
    fn test_exclamation_token() {
        let code = "! !=";
        let mut token: Token;
        let mut lexer = Lexer::new(code);

        token = lexer.next();
        assert_eq!(token.kind, Kind::Exclamation);

        // white space
        lexer.next();

        token = lexer.next();
        assert_eq!(token.kind, Kind::ExclamationEquals);
    }

    #[test]
    fn test_equals_token() {
        let code = "= ==";
        let mut token: Token;
        let mut lexer = Lexer::new(code);

        token = lexer.next();
        assert_eq!(token.kind, Kind::Equals);

        // white space
        lexer.next();

        token = lexer.next();
        assert_eq!(token.kind, Kind::EqualsEquals);
    }

    #[test]
    fn test_greater_than_token() {
        let code = "> >= >>";
        let mut token: Token;
        let mut lexer = Lexer::new(code);

        token = lexer.next();
        assert_eq!(token.kind, Kind::GreaterThan);

        // white space
        lexer.next();

        token = lexer.next();
        assert_eq!(token.kind, Kind::GreaterThanEquals);

        // white space
        lexer.next();

        token = lexer.next();
        assert_eq!(token.kind, Kind::GreaterThanGreaterThan);
    }

    #[test]
    fn test_less_than_token() {
        let code = "< <= <<";
        let mut token: Token;
        let mut lexer = Lexer::new(code);

        token = lexer.next();
        assert_eq!(token.kind, Kind::LessThan);

        // white space
        lexer.next();

        token = lexer.next();
        assert_eq!(token.kind, Kind::LessThanEquals);

        // white space
        lexer.next();

        token = lexer.next();
        assert_eq!(token.kind, Kind::LessThanLessThan);
    }

    #[test]
    fn test_tilde_token() {
        let code = "~ ~=";
        let mut token: Token;
        let mut lexer = Lexer::new(code);

        token = lexer.next();
        assert_eq!(token.kind, Kind::Tilde);

        // white space
        lexer.next();

        token = lexer.next();
        assert_eq!(token.kind, Kind::TildeEquals);
    }

    #[test]
    fn test_circumflex_token() {
        let code = "^ ^=";
        let mut token: Token;
        let mut lexer = Lexer::new(code);

        token = lexer.next();
        assert_eq!(token.kind, Kind::Circumflex);

        // white space
        lexer.next();

        token = lexer.next();
        assert_eq!(token.kind, Kind::CircumflexEquals);
    }

    #[test]
    fn test_identifier_token() {
        let code = "while true for variable";
        let mut token: Token;
        let mut lexer = Lexer::new(code);

        token = lexer.next();
        assert_eq!(token.kind, Kind::While);

        // white space
        lexer.next();

        token = lexer.next();
        assert_eq!(token.kind, Kind::Boolean);

        // white space
        lexer.next();

        token = lexer.next();
        assert_eq!(token.kind, Kind::For);

        // white space
        lexer.next();

        token = lexer.next();
        assert_eq!(token.kind, Kind::Identifier);
    }

    #[test]
    fn test_number_token() {
        let mut code = "23";
        let mut token: Token;
        let mut lexer = Lexer::new(code);

        token = lexer.next();
        assert_eq!(token.kind, Kind::Number);
        assert_eq!(token.value, "23");

        code = "23.2";
        lexer = Lexer::new(code);

        token = lexer.next();
        assert_eq!(token.kind, Kind::Number);
        assert_eq!(token.value, "23.2");

        code = "2.";
        lexer = Lexer::new(code);

        token = lexer.next();
        assert_eq!(token.kind, Kind::Bad);
    }

    #[test]
    fn test_char_token() {
        let mut code = "'c'";
        let mut token: Token;
        let mut lexer = Lexer::new(code);

        token = lexer.next();
        assert_eq!(token.kind, Kind::Char);
        assert_eq!(token.value, "c");

        code = "'\\0'";
        lexer = Lexer::new(code);

        token = lexer.next();
        assert_eq!(token.kind, Kind::Char);

        code = "'c";
        lexer = Lexer::new(code);
        token = lexer.next();

        assert_eq!(token.kind, Kind::Bad);

        code = "\\";
        lexer = Lexer::new(code);
        token = lexer.next();

        assert_eq!(token.kind, Kind::Bad);
    }

    #[test]
    fn test_string_token() {
        let mut code = "\"test string\"";
        let mut token: Token;
        let mut lexer = Lexer::new(code);

        token = lexer.next();
        assert_eq!(token.kind, Kind::String);
        assert_eq!(token.value, "test string");

        code = "\"test string 2\"";
        lexer = Lexer::new(code);

        token = lexer.next();
        assert_eq!(token.kind, Kind::String);
        assert_eq!(token.value, "test string 2");

        code = "'\"test string";
        lexer = Lexer::new(code);

        token = lexer.next();
        assert_eq!(token.kind, Kind::Bad);
    }
}
