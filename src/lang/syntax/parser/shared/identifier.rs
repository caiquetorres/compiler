use crate::lang::syntax::lexer::token::Token;
use crate::lang::syntax::tree_display::TreeDisplay;

pub struct Identifier(pub Token);

impl TreeDisplay for Identifier {
    fn display(&self, layer: usize) {
        let value = self.0.value.as_ref().unwrap();
        println!("{}Identifier ({})", " ".repeat(layer), value);
    }
}
