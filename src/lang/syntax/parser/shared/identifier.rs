use crate::lang::syntax::lexer::token::Token;
use crate::lang::syntax::tree_display::TreeDisplay;

pub struct Identifier {
    pub token: Token,
}

impl Identifier {
    pub fn new(token: Token) -> Self {
        Self { token }
    }
}

impl TreeDisplay for Identifier {
    fn display(&self, layer: usize) {
        let value = self.token.value.clone();
        println!("{}Identifier ({})", " ".repeat(layer), value);
    }
}
