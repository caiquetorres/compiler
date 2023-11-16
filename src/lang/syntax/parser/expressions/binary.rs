use super::expression::Expression;
use crate::lang::syntax::{lexer::token::Token, tree_display::TreeDisplay};

pub struct BinaryOperator(pub Token);

impl TreeDisplay for BinaryOperator {
    fn display(&self, layer: usize) {
        let value = self.0.value.clone();
        println!("{}BinaryOperator ({})", " ".repeat(layer), value);
    }
}

pub struct Binary {
    pub left: Box<Expression>,
    pub operator: BinaryOperator,
    pub right: Box<Expression>,
}

impl Binary {
    pub fn new(left: Expression, operator: BinaryOperator, right: Expression) -> Self {
        Self {
            left: Box::new(left),
            operator,
            right: Box::new(right),
        }
    }
}

impl TreeDisplay for Binary {
    fn display(&self, layer: usize) {
        println!("{}BinaryExpression", " ".repeat(layer));
        self.left.display(layer + 2);
        self.operator.display(layer + 2);
        self.right.display(layer + 2);
    }
}