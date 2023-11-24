use super::expression::Expression;
use crate::lang::syntax::tree_display::TreeDisplay;

#[derive(Clone, Debug)]
pub struct Parenthesized {
    pub expression: Box<Expression>,
}

impl Parenthesized {
    pub fn new(expression: Expression) -> Self {
        Self {
            expression: Box::new(expression),
        }
    }
}

impl TreeDisplay for Parenthesized {
    fn display(&self, layer: usize) {
        println!("{}ParenthesizedExpression", " ".repeat(layer));
        self.expression.display(layer + 1);
    }
}
