use std::{cell::RefCell, rc::Rc};

use crate::lang::{
    semantic::{scope::Scope, semantic_error::SemanticError, semantic_type::SemanticType},
    syntax::expressions::{expression::Expression, literal::Literal},
};

use super::{
    array_analyzer::ArrayAnalyzer, binary_analyzer::BinaryAnalyzer,
    expression_meta_analyzer::ExpressionMetaAnalyzer, identifier_analyzer::IdentifierAnalyzer,
    parenthesized_analyzer::ParenthesizedAnalyzer, range_analyzer::RangeAnalyzer,
    unary_analyzer::UnaryAnalyzer,
};

pub struct ExpressionAnalyzer {
    pub changeable: bool,
    pub return_type: SemanticType,
    pub diagnosis: Vec<SemanticError>,
}

impl ExpressionAnalyzer {
    pub fn analyze(expression: &Expression, scope: Rc<RefCell<Scope>>) -> Self {
        let changeable: bool;
        let return_type: SemanticType;
        let mut diagnosis: Vec<SemanticError> = vec![];

        match expression {
            Expression::Array(array, meta) => {
                let analyzer = ArrayAnalyzer::analyze(array, Rc::clone(&scope));
                diagnosis.extend(analyzer.diagnosis);

                changeable = false;

                if let Some(meta) = &meta {
                    let analyzer = ExpressionMetaAnalyzer::analyze(
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
                let analyzer =
                    ParenthesizedAnalyzer::analyze(parenthesized, meta, Rc::clone(&scope));
                diagnosis.extend(analyzer.diagnosis);

                changeable = analyzer.changeable;
                return_type = analyzer.return_type;
            }
            Expression::Identifier(identifier, meta) => {
                let analyzer = IdentifierAnalyzer::analyze(identifier, meta, Rc::clone(&scope));
                diagnosis.extend(analyzer.diagnosis);

                changeable = analyzer.changeable;
                return_type = analyzer.return_type;
            }
            Expression::Literal(literal) => {
                changeable = false;

                match literal {
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
                }
            }
            Expression::Unary(unary) => {
                let analyzer = UnaryAnalyzer::analyze(unary, Rc::clone(&scope));
                diagnosis.extend(analyzer.diagnosis);

                changeable = false;
                return_type = analyzer.return_type;
            }
            Expression::Range(range) => {
                let analyzer = RangeAnalyzer::analyze(range, Rc::clone(&scope));
                diagnosis.extend(analyzer.diagnosis);

                changeable = false;
                return_type = analyzer.return_type;
            }
            Expression::Binary(binary) => {
                let analyzer = BinaryAnalyzer::analyze(binary, Rc::clone(&scope));
                diagnosis.extend(analyzer.diagnosis);

                changeable = false;
                return_type = analyzer.return_type;
            }
        }

        Self {
            changeable,
            return_type,
            diagnosis,
        }
    }
}
