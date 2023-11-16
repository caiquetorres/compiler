use super::statement::Statement;

use crate::lang::syntax::{
    parser::{expressions::expression::Expression, shared::identifier::Identifier},
    tree_display::TreeDisplay,
};

pub struct For {
    pub identifier: Identifier,
    pub expression: Expression,
    pub statement: Box<Statement>,
}

impl For {
    pub fn new(identifier: Identifier, expression: Expression, statement: Box<Statement>) -> Self {
        Self {
            identifier,
            expression,
            statement,
        }
    }
}

impl TreeDisplay for For {
    fn display(&self, layer: usize) {
        println!(
            "{}ForStatement ({})",
            " ".repeat(layer),
            self.identifier.name
        );

        self.expression.display(layer + 2);
        self.statement.display(layer + 2);
    }
}
