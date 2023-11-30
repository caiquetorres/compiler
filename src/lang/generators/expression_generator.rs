use std::{cell::RefCell, rc::Rc};

use crate::lang::{
    generators::expression_meta_generator::ExpressionMetaGenerator,
    semantic::{expressions::array_analyzer::ArrayAnalyzer, scope::Scope},
    syntax::expressions::{expression::Expression, literal::Literal},
};

use super::{array_generator::ArrayGenerator, c_code_generator2::CCode};

pub struct ExpressionGenerator;

impl ExpressionGenerator {
    pub fn generate(
        expression: &Expression,
        scope: Rc<RefCell<Scope>>,
        ccode: &mut CCode,
    ) -> String {
        match expression {
            Expression::Array(array, meta) => {
                let ArrayAnalyzer { return_type, .. } =
                    ArrayAnalyzer::analyze(array, Rc::clone(&scope));

                ArrayGenerator::generate_expression(
                    &return_type,
                    &array.expressions,
                    meta,
                    scope,
                    ccode,
                )
            }
            Expression::Unary(unary) => {
                format!(
                    "{}{}",
                    unary.operator.token.value,
                    Self::generate(&unary.expression, Rc::clone(&scope), ccode)
                )
            }
            Expression::Binary(binary) => {
                format!(
                    "{}{}{}",
                    Self::generate(&binary.left, Rc::clone(&scope), ccode),
                    binary.operator.token.value,
                    Self::generate(&binary.right, Rc::clone(&scope), ccode)
                )
            }
            Expression::Parenthesized(parenthesized, meta) => {
                if let Some(meta) = &meta {
                    format!(
                        "({}){}",
                        Self::generate(&parenthesized.expression, Rc::clone(&scope), ccode),
                        ExpressionMetaGenerator::generate(meta, Rc::clone(&scope), ccode)
                    )
                } else {
                    format!(
                        "({})",
                        Self::generate(&parenthesized.expression, Rc::clone(&scope), ccode)
                    )
                }
            }
            Expression::Identifier(identifier, meta) => {
                if let Some(meta) = &meta {
                    format!(
                        "{}{}",
                        identifier.name.clone(),
                        ExpressionMetaGenerator::generate(meta, Rc::clone(&scope), ccode)
                    )
                } else {
                    format!("{}", identifier.name.clone())
                }
            }
            Expression::Literal(literal) => match literal {
                Literal::Number(token) => token.value.clone(),
                Literal::Char(token) => format!("'{}'", token.value),
                Literal::String(token) => format!("\"{}\"", token.value),
                Literal::Boolean(token) => match &token.value[..] {
                    "true" => "1".to_string(),
                    _ => "0".to_string(),
                },
            },
            _ => panic!(),
        }
    }
}
