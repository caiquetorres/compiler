use std::{cell::RefCell, rc::Rc};

use crate::lang::syntax::{
    lexer::token_kind::TokenKind,
    parser::expressions::{expression::Expression, literal::Literal},
};

use super::{lang_type::LangType, scope::Scope, semantic_error::SemanticError, symbol::Symbol};

pub struct ExpressionAnalyzer {
    pub return_type: LangType,
    pub diagnosis: Vec<SemanticError>,
}

impl ExpressionAnalyzer {
    pub fn analyze(expression: &Expression, scope: Rc<RefCell<Scope>>) -> Self {
        let mut return_type = LangType::Any;
        let mut diagnosis: Vec<SemanticError> = vec![];

        match expression {
            Expression::Parenthesized(parenthesized) => {
                let analyzer = Self::analyze(&parenthesized.expression, Rc::clone(&scope));
                return_type = analyzer.return_type;
            }
            Expression::Identifier(identifier) => {
                let identifier_name = identifier.name.clone();

                if let Some(symbol) = scope.borrow().get(&identifier_name) {
                    match symbol {
                        Symbol::Variable { symbol_type, .. }
                        | Symbol::Const { symbol_type, .. }
                        | Symbol::Parameter { symbol_type, .. } => {
                            return_type = symbol_type.clone();
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
                Literal::String(_) => return_type = LangType::String,
                Literal::Char(_) => return_type = LangType::Char,
                Literal::Boolean(_) => return_type = LangType::Bool,
                Literal::Number(token) => {
                    return_type = if token.value.contains(".") {
                        LangType::F32
                    } else {
                        LangType::I32
                    }
                }
            },
            Expression::Unary(unary) => {
                let analyzer = Self::analyze(&unary.expression, Rc::clone(&scope));

                if let TokenKind::Tilde = &unary.operator.token.kind {
                    if analyzer.return_type.is_integer() {
                        return_type = analyzer.return_type;
                    } else {
                        diagnosis.push(SemanticError::UnaryOperatorOnlyApplicableToInteger);
                    }
                } else if let TokenKind::Plus | TokenKind::Minus = &unary.operator.token.kind {
                    if analyzer.return_type.is_number() {
                        return_type = analyzer.return_type;
                    } else {
                        diagnosis.push(SemanticError::UnaryOperatorOnlyApplicableToNumbers);
                    }
                } else {
                    if analyzer.return_type == LangType::Bool {
                        return_type = analyzer.return_type;
                    } else {
                        diagnosis.push(SemanticError::UnaryOperatorOnlyApplicableToBooleans);
                    }
                }
            }
            Expression::Range(range) => {
                let analyzer = Self::analyze(&range.left, Rc::clone(&scope));
                let left_return_type = analyzer.return_type;

                let analyzer = Self::analyze(&range.right, Rc::clone(&scope));
                let right_return_type = analyzer.return_type;

                if let TokenKind::DotDot | TokenKind::DotDotEquals = &range.operator.token.kind {
                    if left_return_type.is_number() && right_return_type.is_number() {
                        return_type = LangType::Range;
                    } else {
                        diagnosis.push(SemanticError::InvalidRangeOperands)
                    }
                }
            }
            Expression::Binary(binary) => {
                let analyzer = Self::analyze(&binary.left, Rc::clone(&scope));
                let left_return_type = analyzer.return_type;

                let analyzer = Self::analyze(&binary.right, Rc::clone(&scope));
                let right_return_type = analyzer.return_type;

                match &binary.operator.token.kind {
                    TokenKind::EqualsEquals | TokenKind::ExclamationEquals => {
                        if left_return_type.is_number() && right_return_type.is_number() {
                            return_type = LangType::Bool;
                        } else if left_return_type == right_return_type {
                            return_type = LangType::Bool;
                        } else {
                            diagnosis.push(SemanticError::EqualityTypeMismatch)
                        }
                    }
                    TokenKind::Plus | TokenKind::Minus | TokenKind::Star | TokenKind::Slash => {
                        if left_return_type.is_number() && right_return_type.is_number() {
                            return_type = LangType::number_type_precedence(vec![
                                left_return_type,
                                right_return_type,
                            ]);
                        } else {
                            diagnosis.push(SemanticError::InvalidOperator)
                        }
                    }
                    TokenKind::Mod
                    | TokenKind::Ampersand
                    | TokenKind::Pipe
                    | TokenKind::Tilde
                    | TokenKind::Circumflex => {
                        if left_return_type.is_integer() && right_return_type.is_integer() {
                            return_type = LangType::number_type_precedence(vec![
                                left_return_type,
                                right_return_type,
                            ]);
                        } else {
                            diagnosis.push(SemanticError::InvalidOperator)
                        }
                    }
                    TokenKind::GreaterThan
                    | TokenKind::GreaterThanEquals
                    | TokenKind::LessThan
                    | TokenKind::LessThanEquals => {
                        if left_return_type.is_number() && right_return_type.is_number() {
                            return_type = LangType::Bool;
                        } else {
                            diagnosis.push(SemanticError::InvalidOperator)
                        }
                    }
                    TokenKind::AmpersandAmpersand | TokenKind::PipePipe => {
                        if left_return_type == LangType::Bool && right_return_type == LangType::Bool
                        {
                            return_type = LangType::Bool;
                        } else {
                            diagnosis.push(SemanticError::InvalidOperator)
                        }
                    }
                    _ => unreachable!(),
                }
            }
        }

        Self {
            return_type,
            diagnosis,
        }
    }
}