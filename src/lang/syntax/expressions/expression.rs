use std::fmt::Display;

use super::array::Array;
use super::binary::Binary;
use super::literal::Literal;
use super::parenthesized::Parenthesized;
use super::range::Range;
use super::unary::Unary;
use crate::lang::syntax::shared::identifier::Identifier;
use crate::lang::syntax::tree_display::TreeDisplay;

#[derive(Clone, Debug)]
pub enum ExpressionMeta {
    Index(Box<Expression>, Box<Option<ExpressionMeta>>),
    Call(Vec<Expression>, Box<Option<ExpressionMeta>>),
}

impl TreeDisplay for ExpressionMeta {
    fn display(&self, layer: usize) {
        match &self {
            Self::Index(expression, meta) => {
                println!("{}Index", "  ".repeat(layer));
                expression.display(layer + 1);

                if let Some(meta) = meta.as_ref() {
                    meta.display(layer + 1);
                }
            }
            Self::Call(expressions, meta) => {
                println!("{}Call", "  ".repeat(layer));

                for expression in expressions {
                    expression.display(layer + 1);
                }

                if let Some(meta) = meta.as_ref() {
                    meta.display(layer + 1);
                }
            }
        }
    }
}

#[derive(Clone, Debug)]
pub enum Expression {
    Identifier(Identifier, Option<ExpressionMeta>),
    Literal(Literal),
    Unary(Unary),
    Binary(Binary),
    Parenthesized(Parenthesized, Option<ExpressionMeta>),
    Range(Range),
    Array(Array),
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
            Self::Array(_) => write!(f, "array expression"),
        }
    }
}

impl TreeDisplay for Expression {
    fn display(&self, layer: usize) {
        match self {
            Self::Identifier(identifier, meta) => {
                identifier.display(layer);

                if let Some(meta) = meta {
                    meta.display(layer + 1);
                }
            }
            Self::Literal(literal) => literal.display(layer),
            Self::Unary(unary) => unary.display(layer),
            Self::Binary(binary) => binary.display(layer),
            Self::Parenthesized(parenthesized, meta) => {
                parenthesized.display(layer);

                if let Some(meta) = meta {
                    meta.display(layer + 1);
                }
            }
            Self::Range(range) => range.display(layer),
            Self::Array(array) => array.display(layer),
        }
    }
}
