use super::expression::Expression;
use crate::lang::syntax::{lexer::token::Token, tree_display::TreeDisplay};

pub struct UnaryOperator(pub Token);

impl TreeDisplay for UnaryOperator {
    fn display(&self, _: usize) {
        let value = self.0.value.clone();
        println!("UnaryOperator ({})", value);
    }
}

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
