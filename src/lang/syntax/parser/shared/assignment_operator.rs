use crate::lang::{lexer::token::Token, syntax::tree_display::TreeDisplay};

#[derive(Clone, Debug)]
pub struct AssignmentOperator {
    pub name: String,
    pub token: Token,
}

impl AssignmentOperator {
    pub fn new(token: Token) -> Self {
        let name = token.value.clone();
        Self { name, token }
    }
}

impl TreeDisplay for AssignmentOperator {
    fn display(&self, layer: usize) {
        let value = self.token.value.clone();
        println!("{}AssignmentOperator ({})", "  ".repeat(layer), value);
    }
}
