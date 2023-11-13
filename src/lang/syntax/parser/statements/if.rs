use super::statement::Statement;
use crate::lang::syntax::{parser::expressions::expression::Expression, tree_display::TreeDisplay};

pub struct Else(pub Box<Statement>);

impl TreeDisplay for Else {
    fn display(&self, layer: usize) {
        println!("{}ElseStatement", " ".repeat(layer));
        self.0.display(layer + 2)
    }
}

pub struct If(pub Expression, pub Box<Statement>, pub Option<Else>);

impl TreeDisplay for If {
    fn display(&self, layer: usize) {
        println!("{}IfStatement", " ".repeat(layer));
        self.0.display(layer + 2);
        self.1.display(layer + 2);

        if let Some(r#else) = &self.2 {
            println!("{}ElseStatement", " ".repeat(layer));
            r#else.0.display(layer + 2)
        }
    }
}
