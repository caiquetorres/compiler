use crate::lang::{lexer::token::Token, syntax::tree_display::TreeDisplay};

use super::expression::Expression;

#[derive(Clone)]
pub struct RangeOperator {
    pub token: Token,
}

impl RangeOperator {
    pub fn new(token: Token) -> Self {
        Self { token }
    }
}

impl TreeDisplay for RangeOperator {
    fn display(&self, layer: usize) {
        let value = self.token.value.clone();
        println!("{}RangeOperator ({})", " ".repeat(layer), value);
    }
}

#[derive(Clone)]
pub struct Range {
    pub left: Box<Expression>,
    pub operator: RangeOperator,
    pub right: Box<Expression>,
}

impl Range {
    pub fn new(left: Expression, operator: RangeOperator, right: Expression) -> Self {
        Self {
            left: Box::new(left),
            operator,
            right: Box::new(right),
        }
    }
}

impl TreeDisplay for Range {
    fn display(&self, layer: usize) {
        println!("{}RangeExpression", " ".repeat(layer));
        self.left.display(layer + 2);
        self.operator.display(layer + 2);
        self.right.display(layer + 2);
    }
}
