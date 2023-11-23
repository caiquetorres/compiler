use std::{cell::RefCell, rc::Rc};

use crate::lang::syntax::parser::statements::r#return::Return;

use super::{
    expression_analyzer::ExpressionAnalyzer, lang_type::LangType, scope::Scope,
    semantic_error::SemanticError,
};

pub struct ReturnAnalyzer {
    pub(crate) diagnosis: Vec<SemanticError>,
}

impl ReturnAnalyzer {
    pub fn analyze(r#return: &Return, scope: Rc<RefCell<Scope>>) -> Self {
        let mut diagnosis: Vec<SemanticError> = vec![];

        if let Some(function_return_type) = scope.borrow().get_return_type() {
            let return_type = match &r#return.expression {
                None => LangType::Void,
                Some(expression) => {
                    let analyzer = ExpressionAnalyzer::analyze(expression, Rc::clone(&scope));

                    diagnosis.extend(analyzer.diagnosis);

                    analyzer.return_type
                }
            };

            if function_return_type != return_type
                && (!function_return_type.is_number() || !return_type.is_number())
            {
                diagnosis.push(SemanticError::ExpectedType {
                    expected: function_return_type,
                    found: return_type,
                })
            }
        } else {
            diagnosis.push(SemanticError::InvalidReturn)
        }

        Self { diagnosis }
    }
}
