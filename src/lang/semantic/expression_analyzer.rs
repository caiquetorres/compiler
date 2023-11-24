use std::{cell::RefCell, rc::Rc};

use crate::lang::syntax::parser::expressions::{
    expression::{Expression, ExpressionMeta},
    literal::Literal,
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

pub struct ExpressionMetaAnalyzer {
    pub return_type: SemanticType,
    pub diagnosis: Vec<SemanticError>,
}

impl ExpressionMetaAnalyzer {
    fn analyze_meta(
        r#type: &SemanticType,
        meta: &ExpressionMeta,
        scope: Rc<RefCell<Scope>>,
    ) -> Self {
        let mut return_type = SemanticType::Any;
        let mut diagnosis: Vec<SemanticError> = vec![];

        match meta {
            ExpressionMeta::Index(expression, meta) => {
                // check if symbol_type is a function, then it can be called

                let analyzer = ExpressionAnalyzer::analyze(expression, Rc::clone(&scope));
                diagnosis.extend(analyzer.diagnosis);

                // TODO: Verify if expression is a number

                if let SemanticType::Array(array_type, _) = r#type {
                    // Safe

                    if let Some(meta) = &meta.as_ref() {
                        let analyzer = ExpressionMetaAnalyzer::analyze_meta(
                            &array_type,
                            &meta,
                            Rc::clone(&scope),
                        );

                        diagnosis.extend(analyzer.diagnosis);
                        return_type = analyzer.return_type;
                    } else {
                        return_type = array_type.as_ref().clone();
                    }
                } else {
                    // Não é um array
                }
            }
            ExpressionMeta::Call(expressions, meta) => {
                // check if symbol is a function, then it can be called

                for expression in expressions {
                    let analyzer = ExpressionAnalyzer::analyze(expression, Rc::clone(&scope));
                    diagnosis.extend(analyzer.diagnosis);
                }

                if let SemanticType::Function(params, function_return_type) = r#type {
                    // Safe

                    if let Some(meta) = &meta.as_ref() {
                        let analyzer = ExpressionMetaAnalyzer::analyze_meta(
                            &function_return_type,
                            &meta,
                            Rc::clone(&scope),
                        );

                        diagnosis.extend(analyzer.diagnosis);
                        return_type = analyzer.return_type;
                    } else {
                        // TODO: Check the expressions position and value (params).

                        if params.len() != expressions.len() {
                            diagnosis.push(SemanticError::InvalidNumberOfParameters);
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
                                    diagnosis.push(SemanticError::InvalidParameterType);
                                }
                            }
                        }

                        return_type = function_return_type.as_ref().clone();
                    }
                } else {
                    // Não é função
                }
            }
        }

        Self {
            return_type,
            diagnosis,
        }
    }
}

pub struct ExpressionAnalyzer {
    pub return_type: SemanticType,
    pub diagnosis: Vec<SemanticError>,
}

impl ExpressionAnalyzer {
    pub fn analyze(expression: &Expression, scope: Rc<RefCell<Scope>>) -> Self {
        let mut return_type = SemanticType::Any;
        let mut diagnosis: Vec<SemanticError> = vec![];

        match expression {
            Expression::Array(array, meta) => {
                let analyzer = ArrayAnalyzer::analyze(array, Rc::clone(&scope));
                diagnosis.extend(analyzer.diagnosis);

                if let Some(meta) = &meta {
                    let analyzer = ExpressionMetaAnalyzer::analyze_meta(
                        &analyzer.return_type,
                        &meta,
                        Rc::clone(&scope),
                    );
                    diagnosis.extend(analyzer.diagnosis);
                    return_type = analyzer.return_type;
                } else {
                    return_type = analyzer.return_type;
                }
            }
            Expression::Parenthesized(parenthesized, meta) => {
                let analyzer = ParenthesizedAnalyzer::analyze(parenthesized, Rc::clone(&scope));
                diagnosis.extend(analyzer.diagnosis);

                if let Some(meta) = &meta {
                    let analyzer = ExpressionMetaAnalyzer::analyze_meta(
                        &analyzer.return_type,
                        &meta,
                        Rc::clone(&scope),
                    );
                    diagnosis.extend(analyzer.diagnosis);
                    return_type = analyzer.return_type;
                } else {
                    return_type = analyzer.return_type;
                }
            }
            Expression::Identifier(identifier, meta) => {
                let identifier_name = identifier.name.clone();

                if let Some(symbol) = scope.borrow().get(&identifier_name) {
                    match symbol {
                        Symbol::Variable { symbol_type, .. }
                        | Symbol::Const { symbol_type, .. }
                        | Symbol::Parameter { symbol_type, .. }
                        | Symbol::Function { symbol_type, .. } => {
                            if let Some(meta) = &meta {
                                let analyzer = ExpressionMetaAnalyzer::analyze_meta(
                                    &symbol_type,
                                    &meta,
                                    Rc::clone(&scope),
                                );
                                diagnosis.extend(analyzer.diagnosis);
                                return_type = analyzer.return_type;
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
