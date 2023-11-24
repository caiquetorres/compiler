use crate::lang::syntax::tree_display::TreeDisplay;

use super::expression::Expression;

#[derive(Clone, Debug)]
pub struct Array {
    pub expressions: Vec<Expression>,
}

impl Array {
    pub fn new(expressions: Vec<Expression>) -> Self {
        Self { expressions }
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
