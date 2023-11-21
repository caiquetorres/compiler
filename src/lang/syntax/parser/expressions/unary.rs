use super::expression::Expression;
use crate::lang::syntax::{lexer::token::Token, tree_display::TreeDisplay};

#[derive(Clone)]
pub struct UnaryOperator {
    pub token: Token,
}

impl UnaryOperator {
    pub fn new(token: Token) -> Self {
        Self { token }
    }
}

impl TreeDisplay for UnaryOperator {
    fn display(&self, layer: usize) {
        let value = self.token.value.clone();
        println!("{}UnaryOperator ({})", " ".repeat(layer), value);
    }
}

#[derive(Clone)]
pub struct Unary {
    pub operator: UnaryOperator,
    pub expression: Box<Expression>,
}

impl Unary {
    pub fn new(operator: UnaryOperator, expression: Expression) -> Self {
        Self {
            operator,
            expression: Box::new(expression),
        }
    }
}

impl TreeDisplay for Unary {
    fn display(&self, layer: usize) {
        println!("{}UnaryExpression", " ".repeat(layer));
        self.operator.display(layer + 1);
        self.expression.display(layer + 1);
    }
}
