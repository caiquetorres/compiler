use crate::lang::syntax::{
    parser::{expressions::expression::Expression, shared::identifier::Identifier},
    tree_display::TreeDisplay,
};

use super::statement::Statement;

pub struct For(pub Identifier, pub Expression, pub Box<Statement>);

impl TreeDisplay for For {
    fn display(&self, layer: usize) {
        println!("{}ForStatement ({})", " ".repeat(layer), self.0.token.value);

        self.1.display(layer + 2);
        self.2.display(layer + 2);
    }
}
