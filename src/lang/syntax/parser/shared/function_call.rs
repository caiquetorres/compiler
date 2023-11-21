use super::identifier::Identifier;
use crate::lang::syntax::parser::expressions::expression::Expression;
use crate::lang::syntax::tree_display::TreeDisplay;

#[derive(Clone)]
pub struct Params {
    pub expressions: Vec<Expression>,
}

impl Params {
    pub fn new(expressions: Vec<Expression>) -> Self {
        Self { expressions }
    }
}

impl TreeDisplay for Params {
    fn display(&self, layer: usize) {
        println!("{}Params", " ".repeat(layer));
        for expression in &self.expressions {
            expression.display(layer + 2);
        }
    }
}

#[derive(Clone)]
pub struct FunctionCall {
    pub identifier: Identifier,
    pub params: Params,
}

impl FunctionCall {
    pub fn new(identifier: Identifier, params: Params) -> Self {
        Self { identifier, params }
    }
}

impl TreeDisplay for FunctionCall {
    fn display(&self, layer: usize) {
        println!("{}FunctionCallExpression", " ".repeat(layer));
        self.identifier.display(layer + 2);
        self.params.display(layer + 2);
    }
}
