use super::expression::Expression;
use crate::lang::syntax::{lexer::token::Token, tree_display::TreeDisplay};

#[derive(Clone)]
pub struct BinaryOperator {
    pub token: Token,
}

impl BinaryOperator {
    pub fn new(token: Token) -> Self {
        Self { token }
    }
}

impl TreeDisplay for BinaryOperator {
    fn display(&self, layer: usize) {
        let value = self.token.value.clone();
        println!("{}BinaryOperator ({})", " ".repeat(layer), value);
    }
}

#[derive(Clone)]
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
