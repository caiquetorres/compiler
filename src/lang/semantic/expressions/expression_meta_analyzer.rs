use std::cell::RefCell;
use std::rc::Rc;

use crate::lang::position::Positioned;
use crate::lang::semantic::scope::Scope;
use crate::lang::semantic::semantic_error::SemanticError;
use crate::lang::semantic::semantic_type::SemanticType;
use crate::lang::syntax::expressions::expression::ExpressionMeta;

use super::expression_analyzer::ExpressionAnalyzer;

pub struct ExpressionMetaAnalyzer {
    pub changeable: bool,
    pub return_type: SemanticType,
    pub diagnosis: Vec<SemanticError>,
}

impl ExpressionMetaAnalyzer {
    pub fn analyze(
        r#type: &SemanticType,
        meta: &ExpressionMeta,
        scope: Rc<RefCell<Scope>>,
    ) -> Self {
        let changeable: bool;
        let mut return_type = SemanticType::Any;
        let mut diagnosis: Vec<SemanticError> = vec![];

        match meta {
            ExpressionMeta::Index(expression, meta, position) => {
                let analyzer = ExpressionAnalyzer::analyze(expression, Rc::clone(&scope));
                diagnosis.extend(analyzer.diagnosis);

                match r#type {
                    SemanticType::Any => changeable = true,
                    SemanticType::String => changeable = true,
                    SemanticType::Array(array_type, _) => {
                        if let Some(meta) = &meta.as_ref() {
                            let analyzer = ExpressionMetaAnalyzer::analyze(
                                &array_type,
                                &meta,
                                Rc::clone(&scope),
                            );

                            diagnosis.extend(analyzer.diagnosis);

                            changeable = analyzer.changeable;
                            return_type = analyzer.return_type;
                        } else {
                            changeable = true;
                            return_type = array_type.as_ref().clone();
                        }
                    }
                    _ => {
                        diagnosis.push(SemanticError::IdentifierNotIndexable {
                            position: *position,
                        });
                        changeable = true;
                    }
                }
            }
            ExpressionMeta::Call(expressions, meta, position) => {
                for expression in expressions {
                    let analyzer = ExpressionAnalyzer::analyze(expression, Rc::clone(&scope));
                    diagnosis.extend(analyzer.diagnosis);
                }

                match r#type {
                    SemanticType::Any => changeable = true,
                    SemanticType::Function(params, function_return_type) => {
                        if params.len() != expressions.len() {
                            diagnosis.push(SemanticError::InvalidNumberOfParameters {
                                expected: params.len(),
                                found: expressions.len(),
                                position: *position,
                            });
                        } else {
                            for i in 0..params.len() {
                                let expected_param_type = params.get(i).unwrap();
                                let expression = expressions.get(i).unwrap();

                                let analyzer =
                                    ExpressionAnalyzer::analyze(expression, Rc::clone(&scope));

                                if expected_param_type.clone() != analyzer.return_type
                                    && (!expected_param_type.is_number()
                                        || !analyzer.return_type.is_number())
                                {
                                    diagnosis.push(SemanticError::InvalidParameterType {
                                        expected: expected_param_type.clone(),
                                        found: analyzer.return_type,
                                        position: expression.get_position(),
                                    });
                                }
                            }
                        }

                        if let Some(meta) = &meta.as_ref() {
                            let analyzer = ExpressionMetaAnalyzer::analyze(
                                &function_return_type,
                                &meta,
                                Rc::clone(&scope),
                            );

                            diagnosis.extend(analyzer.diagnosis);

                            changeable = analyzer.changeable;
                            return_type = analyzer.return_type;
                        } else {
                            changeable = false;
                            return_type = function_return_type.as_ref().clone();
                        }
                    }
                    _ => {
                        diagnosis.push(SemanticError::IdentifierNotCallable {
                            position: *position,
                        });
                        changeable = true;
                    }
                }
            }
        }

        Self {
            changeable,
            return_type,
            diagnosis,
        }
    }
}
