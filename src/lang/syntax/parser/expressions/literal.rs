use crate::lang::syntax::lexer::token::Token;
use crate::lang::syntax::tree_display::TreeDisplay;

#[derive(Clone)]
pub enum Literal {
    Boolean(Token),
    Char(Token),
    Number(Token),
}

impl TreeDisplay for Literal {
    fn display(&self, layer: usize) {
        // REVIEW: Should we show the type here?
        match self {
            Self::Boolean(token) | Self::Char(token) | Self::Number(token) => {
                println!(
                    "{}Literal ({}) ({})",
                    " ".repeat(layer),
                    token.value,
                    token.kind
                )
            }
        }
    }
}
