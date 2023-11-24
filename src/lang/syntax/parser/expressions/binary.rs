use super::expression::Expression;
use crate::lang::{lexer::token::Token, syntax::tree_display::TreeDisplay};

#[derive(Clone, Debug)]
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
        println!("{}BinaryOperator ({})", "  ".repeat(layer), value);
    }
}

#[derive(Clone, Debug)]
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
        println!("{}BinaryExpression", "  ".repeat(layer));
        self.left.display(layer + 1);
        self.operator.display(layer + 1);
        self.right.display(layer + 1);
    }
}
