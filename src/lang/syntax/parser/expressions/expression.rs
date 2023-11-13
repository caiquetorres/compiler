use super::binary::Binary;
use super::literal::Literal;
use super::parenthesized::Parenthesized;
use super::unary::Unary;
use crate::lang::syntax::parser::shared::function_call::FunctionCall;
use crate::lang::syntax::parser::shared::identifier::Identifier;
use crate::lang::syntax::tree_display::TreeDisplay;

pub enum Expression {
    Identifier(Identifier),
    Literal(Literal),
    Unary(Unary),
    Binary(Binary),
    Parenthesized(Parenthesized),
    FunctionCall(FunctionCall),
}

impl TreeDisplay for Expression {
    fn display(&self, layer: usize) {
        match self {
            Self::Identifier(id) => id.display(layer),
            Self::Literal(literal) => literal.display(layer),
            Self::Unary(unary) => unary.display(layer),
            Self::Binary(binary) => binary.display(layer),
            Self::Parenthesized(parenthesized) => parenthesized.display(layer),
            Self::FunctionCall(call) => call.display(layer),
        }
    }
}
