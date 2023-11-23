use std::{cell::RefCell, rc::Rc};

use crate::lang::syntax::parser::{
    expressions::{expression::Expression, literal::Literal},
    shared::identifier::IdentifierMeta,
};

use super::{
    expressions::{
        array_analyzer::ArrayAnalyzer, binary_analyzer::BinaryAnalyzer,
        parenthesized_analyzer::ParenthesizedAnalyzer, range_analyzer::RangeAnalyzer,
        unary_analyzer::UnaryAnalyzer,
    },
    scope::Scope,
    semantic_error::SemanticError,
    semantic_type::SemanticType,
    symbol::Symbol,
};

pub struct ExpressionAnalyzer {
    pub return_type: SemanticType,
    pub diagnosis: Vec<SemanticError>,
}

impl ExpressionAnalyzer {
    pub fn analyze(expression: &Expression, scope: Rc<RefCell<Scope>>) -> Self {
        let mut return_type = SemanticType::Any;
        let mut diagnosis: Vec<SemanticError> = vec![];

        match expression {
            Expression::Array(array) => {
                let analyzer = ArrayAnalyzer::analyze(array, Rc::clone(&scope));
                diagnosis.extend(analyzer.diagnosis);
                return_type = analyzer.return_type;
            }
            Expression::Parenthesized(parenthesized) => {
                let analyzer = ParenthesizedAnalyzer::analyze(parenthesized, Rc::clone(&scope));
                diagnosis.extend(analyzer.diagnosis);
                return_type = analyzer.return_type;
            }
            Expression::Identifier(identifier) => {
                let identifier_name = identifier.name.clone();

                if let Some(symbol) = scope.borrow().get(&identifier_name) {
                    match symbol {
                        Symbol::Variable { symbol_type, .. }
                        | Symbol::Const { symbol_type, .. }
                        | Symbol::Parameter { symbol_type, .. } => {
                            if let Some(meta) = &identifier.meta {
                                match meta {
                                    IdentifierMeta::Index(expression, meta) => {
                                        match symbol_type {
                                            SemanticType::Array(r#type, ..) => {
                                                let analyzer = Self::analyze(
                                                    expression.as_ref(),
                                                    Rc::clone(&scope),
                                                );

                                                diagnosis.extend(analyzer.diagnosis);

                                                if let None = meta.as_ref() {
                                                    return_type = r#type.as_ref().clone();
                                                }

                                                // TODO: Recursion for checking indexes.
                                            }
                                            _ => diagnosis
                                                .push(SemanticError::IdentifierNotIndexable),
                                        }
                                    }
                                }
                            } else {
                                return_type = symbol_type.clone();
                            }
                        }
                        _ => {
                            diagnosis.push(SemanticError::IdentifierNotVariableConstOrParam);
                        }
                    }
                } else {
                    diagnosis.push(SemanticError::IdentifierNotFound);
                }
            }
            Expression::FunctionCall(function_call) => {
                let function_name = function_call.identifier.name.clone();

                if let Some(symbol) = scope.borrow().get(&function_name) {
                    match symbol {
                        Symbol::Function {
                            params,
                            symbol_type,
                            ..
                        } => {
                            return_type = symbol_type.clone();

                            if params.len() != function_call.params.expressions.len() {
                                diagnosis.push(SemanticError::InvalidNumberOfParameters);
                            } else {
                                for i in 0..params.len() {
                                    let expected_param_type = params.get(i).unwrap();
                                    let expression =
                                        function_call.params.expressions.get(i).unwrap();

                                    let analyzer = Self::analyze(expression, Rc::clone(&scope));

                                    if expected_param_type.clone() != analyzer.return_type
                                        && (!expected_param_type.is_number()
                                            || !analyzer.return_type.is_number())
                                    {
                                        diagnosis.push(SemanticError::InvalidParameterType);
                                    }
                                }
                            }
                        }
                        _ => {
                            diagnosis.push(SemanticError::IdentifierNotCallable);
                        }
                    }
                } else {
                    diagnosis.push(SemanticError::IdentifierNotFound);
                }
            }
            Expression::Literal(literal) => match literal {
                Literal::String(_) => return_type = SemanticType::String,
                Literal::Char(_) => return_type = SemanticType::Char,
                Literal::Boolean(_) => return_type = SemanticType::Bool,
                Literal::Number(token) => {
                    return_type = if token.value.contains(".") {
                        SemanticType::F32
                    } else {
                        SemanticType::I32
                    }
                }
            },
            Expression::Unary(unary) => {
                let analyzer = UnaryAnalyzer::analyze(unary, Rc::clone(&scope));
                diagnosis.extend(analyzer.diagnosis);
                return_type = analyzer.return_type;
            }
            Expression::Range(range) => {
                let analyzer = RangeAnalyzer::analyze(range, Rc::clone(&scope));
                diagnosis.extend(analyzer.diagnosis);
                return_type = analyzer.return_type;
            }
            Expression::Binary(binary) => {
                let analyzer = BinaryAnalyzer::analyze(binary, Rc::clone(&scope));
                diagnosis.extend(analyzer.diagnosis);
                return_type = analyzer.return_type;
            }
        }

        Self {
            return_type,
            diagnosis,
        }
    }
}
