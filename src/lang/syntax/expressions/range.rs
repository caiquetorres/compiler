use crate::lang::{
    lexer::token::Token,
    position::{Position, Positioned},
    syntax::tree_display::TreeDisplay,
};

use super::expression::Expression;

#[derive(Clone, Debug)]
pub struct RangeOperator {
    pub token: Token,
}

impl RangeOperator {
    pub fn new(token: Token) -> Self {
        Self { token }
    }
}

impl Positioned for RangeOperator {
    fn get_position(&self) -> crate::lang::position::Position {
        self.token.position
    }
}

impl TreeDisplay for RangeOperator {
    fn display(&self, layer: usize) {
        let value = self.token.value.clone();
        println!("{}RangeOperator ({})", "  ".repeat(layer), value);
    }
}

#[derive(Clone, Debug)]
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

impl Positioned for Range {
    fn get_position(&self) -> Position {
        todo!()
    }
}

impl TreeDisplay for Range {
    fn display(&self, layer: usize) {
        println!("{}RangeExpression", "  ".repeat(layer));
        self.left.display(layer + 1);
        self.operator.display(layer + 1);
        self.right.display(layer + 1);
    }
}
