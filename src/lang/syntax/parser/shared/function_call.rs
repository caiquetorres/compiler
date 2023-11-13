use super::identifier::Identifier;
use crate::lang::syntax::parser::expressions::expression::Expression;
use crate::lang::syntax::tree_display::TreeDisplay;

pub struct Params(pub Vec<Expression>);

impl TreeDisplay for Params {
    fn display(&self, layer: usize) {
        println!("{}Params", " ".repeat(layer));
        for expression in &self.0 {
            expression.display(layer + 2);
        }
    }
}

pub struct FunctionCall(pub Identifier, pub Params);

impl TreeDisplay for FunctionCall {
    fn display(&self, layer: usize) {
        println!("{}FunctionCallExpression", " ".repeat(layer));
        self.0.display(layer + 2);
        self.1.display(layer + 2);
    }
}
