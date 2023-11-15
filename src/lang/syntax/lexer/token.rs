use super::token_kind::TokenKind;

#[derive(Clone, Copy, Debug)]
pub struct Position {
    pub position: usize,
    pub column: usize,
    pub line: usize,
}

impl Position {
    pub fn new(position: usize, column: usize, line: usize) -> Self {
        Self {
            position,
            column,
            line,
        }
    }
}

#[derive(Debug)]
pub struct Token {
    pub kind: TokenKind,
    pub position: Position,
    pub value: String,
}

impl Token {
    pub fn new(kind: TokenKind, position: Position, value: &str) -> Self {
        Self {
            kind,
            position,
            value: String::from(value),
        }
    }
}
