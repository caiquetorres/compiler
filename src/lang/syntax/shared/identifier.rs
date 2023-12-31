use crate::lang::lexer::token::Token;
use crate::lang::position::{Position, Positioned};
use crate::lang::syntax::tree_display::TreeDisplay;

#[derive(Clone, Debug)]
pub struct Identifier {
    pub token: Token,
    pub name: String,
}

impl Identifier {
    pub fn new(token: Token) -> Self {
        let name = token.value.clone();
        Self { token, name }
    }
}

impl Positioned for Identifier {
    fn get_position(&self) -> Position {
        self.token.position
    }
}

impl TreeDisplay for Identifier {
    fn display(&self, layer: usize) {
        let value = self.name.clone();
        println!("{}Identifier ({})", "  ".repeat(layer), value);
    }
}
