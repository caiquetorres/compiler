use crate::lang::lexer::token::Token;
use crate::lang::position::{Position, Positioned};
use crate::lang::syntax::tree_display::TreeDisplay;

#[derive(Clone, Debug)]
pub enum Literal {
    Boolean(Token),
    Char(Token),
    Number(Token),
    String(Token),
}

impl Positioned for Literal {
    fn get_position(&self) -> Position {
        match &self {
            Literal::Boolean(token)
            | Literal::Char(token)
            | Literal::String(token)
            | Literal::Number(token) => token.position,
        }
    }
}

impl TreeDisplay for Literal {
    fn display(&self, layer: usize) {
        // REVIEW: Should we show the type here?
        match self {
            Self::Boolean(token) | Self::Number(token) => {
                println!("{}Literal ({})", "  ".repeat(layer), token.value)
            }
            Self::Char(token) => println!("{}Literal ('{}')", "  ".repeat(layer), token.value),
            Self::String(token) => {
                println!("{}Literal (\"{}\")", "  ".repeat(layer), token.value)
            }
        }
    }
}
