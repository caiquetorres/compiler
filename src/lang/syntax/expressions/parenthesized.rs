use super::expression::Expression;
use crate::lang::{
    position::{Position, Positioned},
    syntax::tree_display::TreeDisplay,
};

#[derive(Clone, Debug)]
pub struct Parenthesized {
    pub position: Position,
    pub expression: Box<Expression>,
}

impl Parenthesized {
    pub fn new(expression: Expression, position: Position) -> Self {
        Self {
            expression: Box::new(expression),
            position,
        }
    }
}

impl Positioned for Parenthesized {
    fn get_position(&self) -> Position {
        self.position
    }
}

impl TreeDisplay for Parenthesized {
    fn display(&self, layer: usize) {
        println!("{}ParenthesizedExpression", "  ".repeat(layer));
        self.expression.display(layer + 1);
    }
}
