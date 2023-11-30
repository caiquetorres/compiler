use super::{c_code_generator2::CCode, expression_generator::ExpressionGenerator};

use crate::lang::semantic::scope::Scope;
use crate::lang::syntax::expressions::expression::ExpressionMeta;

use std::{cell::RefCell, rc::Rc};

pub struct ExpressionMetaGenerator;

impl ExpressionMetaGenerator {
    pub fn generate(meta: &ExpressionMeta, scope: Rc<RefCell<Scope>>, ccode: &mut CCode) -> String {
        let mut code = String::new();

        match meta {
            ExpressionMeta::Call(expressions, meta, _) => {
                code.push_str("(");

                for (index, expression) in expressions.iter().enumerate() {
                    code.push_str(&ExpressionGenerator::generate(
                        expression,
                        Rc::clone(&scope),
                        ccode,
                    ));

                    if index != expressions.len() - 1 {
                        code.push_str(",");
                    }
                }

                code.push_str(")");

                if let Some(meta) = meta.as_ref() {
                    code.push_str(&Self::generate(meta, Rc::clone(&scope), ccode));
                }
            }
            ExpressionMeta::Index(expression, meta, _) => {
                code.push_str("[");

                code.push_str(&ExpressionGenerator::generate(
                    expression,
                    Rc::clone(&scope),
                    ccode,
                ));

                code.push_str("]");

                if let Some(meta) = meta.as_ref() {
                    code.push_str(&Self::generate(meta, Rc::clone(&scope), ccode));
                }
            }
        }

        code
    }
}
