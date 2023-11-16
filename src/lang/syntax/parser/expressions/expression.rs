use std::fmt::Display;

use super::binary::Binary;
use super::literal::Literal;
use super::parenthesized::Parenthesized;
use super::range::Range;
use super::unary::Unary;
use crate::lang::syntax::parser::shared::function_call::FunctionCall;
use crate::lang::syntax::parser::shared::identifier::Identifier;
use crate::lang::syntax::tree_display::TreeDisplay;

pub enum Expression {
    Identifier(Identifier),
    FunctionCall(FunctionCall),
    Literal(Literal),
    Unary(Unary),
    Binary(Binary),
    Parenthesized(Parenthesized),
    Range(Range),
}

impl Display for Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Binary(_) => write!(f, "binary expression"),
            Self::Identifier(_) => write!(f, "identifier expression"),
            Self::FunctionCall(_) => write!(f, "function call expression"),
            Self::Literal(_) => write!(f, "literal expression"),
            Self::Unary(_) => write!(f, "unary expression"),
            Self::Parenthesized(_) => write!(f, "parenthesized expression"),
            Self::Range(_) => write!(f, "range expression"),
        }
    }
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
            Self::Range(range) => range.display(layer),
        }
    }
}
