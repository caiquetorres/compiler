use crate::lang::syntax::{lexer::token::Token, tree_display::TreeDisplay};

pub struct AssignmentOperator(pub Token);

impl TreeDisplay for AssignmentOperator {
    fn display(&self, layer: usize) {
        let value = self.0.value.as_ref().unwrap();
        println!("{}AssignmentOperator ({})", " ".repeat(layer), value);
    }
}
