use std::fmt::Display;

use super::array::Array;
use super::binary::Binary;
use super::literal::Literal;
use super::parenthesized::Parenthesized;
use super::range::Range;
use super::unary::Unary;
use crate::lang::syntax::parser::shared::identifier::Identifier;
use crate::lang::syntax::tree_display::TreeDisplay;

#[derive(Clone, Debug)]
pub enum ExpressionMeta {
    Index(Box<Expression>, Box<Option<ExpressionMeta>>),
    Call(Vec<Expression>, Box<Option<ExpressionMeta>>),
}

#[derive(Clone, Debug)]
pub enum Expression {
    Identifier(Identifier, Option<ExpressionMeta>),
    Literal(Literal),
    Unary(Unary),
    Binary(Binary),
    Parenthesized(Parenthesized, Option<ExpressionMeta>),
    Range(Range),
    Array(Array, Option<ExpressionMeta>),
}

impl Display for Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Binary(_) => write!(f, "binary expression"),
            Self::Identifier(_, _) => write!(f, "identifier expression"),
            Self::Literal(_) => write!(f, "literal expression"),
            Self::Unary(_) => write!(f, "unary expression"),
            Self::Parenthesized(_, _) => write!(f, "parenthesized expression"),
            Self::Range(_) => write!(f, "range expression"),
            Self::Array(_, _) => write!(f, "array expression"),
        }
    }
}

impl TreeDisplay for Expression {
    fn display(&self, layer: usize) {
        match self {
            Self::Identifier(id, _) => id.display(layer),
            Self::Literal(literal) => literal.display(layer),
            Self::Unary(unary) => unary.display(layer),
            Self::Binary(binary) => binary.display(layer),
            Self::Parenthesized(parenthesized, _) => parenthesized.display(layer),
            Self::Range(range) => range.display(layer),
            Self::Array(array, _) => array.display(layer),
        }
    }
}
