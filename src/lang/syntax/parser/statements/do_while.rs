use crate::lang::syntax::{parser::expressions::expression::Expression, tree_display::TreeDisplay};

use super::statement::Statement;

pub struct DoWhile(pub Box<Statement>, pub Expression);

impl TreeDisplay for DoWhile {
    fn display(&self, layer: usize) {
        println!("{}DoWhileStatement", " ".repeat(layer));
        self.0.display(layer + 2);
        self.1.display(layer + 2);
    }
}
