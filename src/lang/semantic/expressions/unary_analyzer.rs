use std::cell::RefCell;
use std::rc::Rc;

use crate::lang::lexer::token_kind::TokenKind;
use crate::lang::position::Positioned;
use crate::lang::semantic::scope::Scope;
use crate::lang::semantic::semantic_error::SemanticError;
use crate::lang::semantic::semantic_type::SemanticType;
use crate::lang::syntax::expressions::unary::Unary;

use super::expression_analyzer::ExpressionAnalyzer;

pub struct UnaryAnalyzer {
    pub return_type: SemanticType,
    pub diagnosis: Vec<SemanticError>,
}

impl UnaryAnalyzer {
    pub fn analyze(unary: &Unary, scope: Rc<RefCell<Scope>>) -> Self {
        let mut return_type = SemanticType::Any;
        let mut diagnosis: Vec<SemanticError> = vec![];

        let analyzer = ExpressionAnalyzer::analyze(&unary.expression, Rc::clone(&scope));

        diagnosis.extend(analyzer.diagnosis);

        if let TokenKind::Tilde = &unary.operator.token.kind {
            if analyzer.return_type.is_integer() {
                return_type = analyzer.return_type;
            } else {
                diagnosis.push(SemanticError::InvalidUnaryOperand {
                    found: analyzer.return_type,
                    position: unary.get_position(),
                });
            }
        } else if let TokenKind::Plus | TokenKind::Minus = &unary.operator.token.kind {
            if analyzer.return_type.is_number() {
                return_type = analyzer.return_type;
            } else {
                diagnosis.push(SemanticError::InvalidUnaryOperand {
                    found: analyzer.return_type,
                    position: unary.get_position(),
                });
            }
        } else {
            if analyzer.return_type == SemanticType::Bool {
                return_type = analyzer.return_type;
            } else {
                diagnosis.push(SemanticError::InvalidUnaryOperand {
                    found: analyzer.return_type,
                    position: unary.get_position(),
                });
            }
        }

        Self {
            return_type,
            diagnosis,
        }
    }
}
