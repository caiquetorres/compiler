use crate::lang::syntax::{parser::statements::statement::Statement, tree_display::TreeDisplay};

pub struct Block {
    pub statements: Vec<Statement>,
}

impl Block {
    pub fn new(statements: Vec<Statement>) -> Self {
        Self { statements }
    }
}

impl TreeDisplay for Block {
    fn display(&self, layer: usize) {
        for statement in &self.statements {
            statement.display(layer);
        }
    }
}
