use crate::lang::{lexer::token::Token, syntax::tree_display::TreeDisplay};

#[derive(Clone, Debug)]
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
            | Self::Number(token)
            | Self::String(token) => {
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
