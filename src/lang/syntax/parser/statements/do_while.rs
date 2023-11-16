use crate::lang::syntax::{parser::expressions::expression::Expression, tree_display::TreeDisplay};

use super::statement::Statement;

pub struct DoWhile {
    pub statement: Box<Statement>,
    pub expression: Expression,
}

impl DoWhile {
    pub fn new(statement: Statement, expression: Expression) -> Self {
        Self {
            statement: Box::new(statement),
            expression,
        }
    }
}

impl TreeDisplay for DoWhile {
    fn display(&self, layer: usize) {
        println!("{}DoWhileStatement", " ".repeat(layer));
        self.statement.display(layer + 2);
        self.expression.display(layer + 2);
    }
}
