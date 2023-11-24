use crate::lang::semantic::scope::Scope;
use crate::lang::semantic::semantic_error::SemanticError;
use crate::lang::semantic::symbol::Symbol;
use crate::lang::semantic::{expression_analyzer::ExpressionAnalyzer, semantic_type::SemanticType};
use crate::lang::syntax::parser::shared::function_call::FunctionCall;

use std::{cell::RefCell, rc::Rc};

pub struct FunctionCallAnalyzer {
    pub(crate) diagnosis: Vec<SemanticError>,
}

impl FunctionCallAnalyzer {
    pub fn analyze(function_call: &FunctionCall, scope: Rc<RefCell<Scope>>) -> Self {
        let mut diagnosis: Vec<SemanticError> = vec![];

        let identifier_name = function_call.identifier.name.clone();

        if let Some(symbol) = scope.borrow().get(&identifier_name) {
            if let Symbol::Function { symbol_type, .. } = symbol {
                if let SemanticType::Function(params, ..) = symbol_type {
                    if params.len() != function_call.params.expressions.len() {
                        diagnosis.push(SemanticError::InvalidNumberOfParameters);
                    } else {
                        for i in 0..params.len() {
                            let expected_param_type = params.get(i).unwrap();
                            let expression = function_call.params.expressions.get(i).unwrap();

                            let analyzer =
                                ExpressionAnalyzer::analyze(expression, Rc::clone(&scope));

                            if expected_param_type.clone() != analyzer.return_type
                                && (!expected_param_type.is_number()
                                    || !analyzer.return_type.is_number())
                            {
                                diagnosis.push(SemanticError::InvalidParameterType);
                            }
                        }
                    }
                }
            } else {
                diagnosis.push(SemanticError::IdentifierNotCallable);
            }
        } else {
            diagnosis.push(SemanticError::IdentifierNotFound);
        }

        Self { diagnosis }
    }
}
