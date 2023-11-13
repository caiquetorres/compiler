use super::statement::Statement;
use crate::lang::syntax::{parser::expressions::expression::Expression, tree_display::TreeDisplay};

pub struct While(pub Expression, pub Box<Statement>);

impl TreeDisplay for While {
    fn display(&self, layer: usize) {
        println!("{}WhileStatement", " ".repeat(layer));
        self.0.display(layer + 2);
        self.1.display(layer + 2);
    }
}
