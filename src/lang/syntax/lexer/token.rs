use super::token_kind::TokenKind;
use crate::lang::position::Position;

#[derive(Debug, Clone)]
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
