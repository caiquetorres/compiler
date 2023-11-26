use crate::lang::position::{Position, Positioned};
use crate::lang::syntax::tree_display::TreeDisplay;

use super::expression::Expression;

#[derive(Clone, Debug)]
pub struct Array {
    pub expressions: Vec<Expression>,
    pub position: Position,
}

impl Array {
    pub fn new(expressions: Vec<Expression>, position: Position) -> Self {
        Self {
            expressions,
            position,
        }
    }
}

impl Positioned for Array {
    fn get_position(&self) -> Position {
        self.position
    }
}

impl TreeDisplay for Array {
    fn display(&self, layer: usize) {
        println!("{}ArrayExpression", "  ".repeat(layer));

        for expression in &self.expressions {
            expression.display(layer + 1);
        }
    }
}
