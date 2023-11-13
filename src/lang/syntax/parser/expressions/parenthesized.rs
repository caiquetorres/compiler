use super::expression::Expression;
use crate::lang::syntax::tree_display::TreeDisplay;

pub struct Parenthesized(pub Box<Expression>);

impl TreeDisplay for Parenthesized {
    fn display(&self, layer: usize) {
        println!("{}ParenthesizedExpression", " ".repeat(layer));
        self.0.display(layer + 1);
    }
}
