use super::statement::Statement;
use crate::lang::syntax::{parser::expressions::expression::Expression, tree_display::TreeDisplay};

pub struct While {
    pub expression: Expression,
    pub statement: Box<Statement>,
}

impl While {
    pub fn new(expression: Expression, statement: Statement) -> Self {
        Self {
            expression,
            statement: Box::new(statement),
        }
    }
}

impl TreeDisplay for While {
    fn display(&self, layer: usize) {
        println!("{}WhileStatement", " ".repeat(layer));
        self.expression.display(layer + 2);
        self.statement.display(layer + 2);
    }
}
