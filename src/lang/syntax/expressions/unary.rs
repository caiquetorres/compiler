use super::expression::Expression;
use crate::lang::{lexer::token::Token, syntax::tree_display::TreeDisplay};
use crate::lang::position::{Position, Positioned};

#[derive(Clone, Debug)]
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
        println!("{}UnaryOperator ({})", "  ".repeat(layer), value);
    }
}

#[derive(Clone, Debug)]
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

impl Positioned for Unary {
    fn get_position(&self) -> Position {
        self.operator.token.position
    }
}

impl TreeDisplay for Unary {
    fn display(&self, layer: usize) {
        println!("{}UnaryExpression", "  ".repeat(layer));
        self.operator.display(layer + 1);
        self.expression.display(layer + 1);
    }
}
