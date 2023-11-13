use crate::lang::syntax::{parser::statements::statement::Statement, tree_display::TreeDisplay};

pub struct Block(pub Vec<Statement>);

impl TreeDisplay for Block {
    fn display(&self, layer: usize) {
        for statement in &self.0 {
            statement.display(layer);
        }
    }
}
