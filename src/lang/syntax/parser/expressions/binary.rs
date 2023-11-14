use super::expression::Expression;
use crate::lang::syntax::{lexer::token::Token, tree_display::TreeDisplay};

pub struct BinaryOperator(pub Token);

impl TreeDisplay for BinaryOperator {
    fn display(&self, layer: usize) {
        let value = self.0.value.clone();
        println!("{}BinaryOperator ({})", " ".repeat(layer), value);
    }
}
pub struct Binary(pub Box<Expression>, pub BinaryOperator, pub Box<Expression>);

impl TreeDisplay for Binary {
    fn display(&self, layer: usize) {
        println!("{}BinaryExpression", " ".repeat(layer));
        self.0.display(layer + 2);
        self.1.display(layer + 2);
        self.2.display(layer + 2);
    }
}
