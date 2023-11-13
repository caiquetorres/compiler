use super::expression::Expression;
use crate::lang::syntax::{lexer::token::Token, tree_display::TreeDisplay};

pub struct UnaryOperator(pub Token);

impl TreeDisplay for UnaryOperator {
    fn display(&self, _: usize) {
        let value = self.0.value.as_ref().unwrap();
        println!("UnaryOperator ({})", value);
    }
}

pub struct Unary(pub UnaryOperator, pub Box<Expression>);

impl TreeDisplay for Unary {
    fn display(&self, layer: usize) {
        println!("{}UnaryExpression", " ".repeat(layer));
        self.0.display(layer + 1);
        self.1.display(layer + 1);
    }
}
