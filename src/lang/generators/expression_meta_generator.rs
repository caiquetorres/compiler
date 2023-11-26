use std::{cell::RefCell, rc::Rc};

use crate::lang::{semantic::scope::Scope, syntax::expressions::expression::ExpressionMeta};

use super::expression_generator::ExpressionGenerator;

pub struct ExpressionMetaGenerator;

impl ExpressionMetaGenerator {
    pub fn generate(meta: &ExpressionMeta, scope: Rc<RefCell<Scope>>) -> String {
        let mut code = String::new();

        match meta {
            ExpressionMeta::Call(expressions, meta, _) => {
                code.push_str("(");

                for (index, expression) in expressions.iter().enumerate() {
                    code.push_str(&ExpressionGenerator::generate(
                        expression,
                        Rc::clone(&scope),
                    ));

                    if index != expressions.len() - 1 {
                        code.push_str(",");
                    }
                }

                code.push_str(")");

                if let Some(meta) = meta.as_ref() {
                    code.push_str(&Self::generate(meta, Rc::clone(&scope)));
                }
            }
            ExpressionMeta::Index(expression, meta, _) => {
                code.push_str("[");

                code.push_str(&ExpressionGenerator::generate(
                    expression,
                    Rc::clone(&scope),
                ));

                code.push_str("]");

                if let Some(meta) = meta.as_ref() {
                    code.push_str(&Self::generate(meta, Rc::clone(&scope)));
                }
            }
        }

        code
    }
}
