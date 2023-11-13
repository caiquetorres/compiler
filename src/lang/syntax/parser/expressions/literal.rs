use crate::lang::syntax::lexer::token::Token;
use crate::lang::syntax::tree_display::TreeDisplay;

pub enum Literal {
    Boolean(Token),
    Char(Token),
    Number(Token),
    String(Token),
}

impl TreeDisplay for Literal {
    fn display(&self, layer: usize) {
        // REVIEW: Should we show the type here?
        match self {
            Self::Boolean(token)
            | Self::Char(token)
            | Self::String(token)
            | Self::Number(token) => {
                println!(
                    "{}Literal ({}) ({})",
                    " ".repeat(layer),
                    token.value.as_ref().unwrap(),
                    token.kind
                )
            }
        }
    }
}
