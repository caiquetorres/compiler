use std::{cell::RefCell, rc::Rc};

use crate::lang::lexer::token_kind::TokenKind;
use crate::lang::position::Positioned;
use crate::lang::semantic::scope::Scope;
use crate::lang::semantic::semantic_error::SemanticError;
use crate::lang::semantic::semantic_type::SemanticType;
use crate::lang::syntax::expressions::range::Range;

use super::expression_analyzer::ExpressionAnalyzer;

pub struct RangeAnalyzer {
    pub return_type: SemanticType,
    pub diagnosis: Vec<SemanticError>,
}

impl RangeAnalyzer {
    pub fn analyze(range: &Range, scope: Rc<RefCell<Scope>>) -> Self {
        let mut return_type = SemanticType::Any;
        let mut diagnosis: Vec<SemanticError> = vec![];

        let analyzer = ExpressionAnalyzer::analyze(&range.left, Rc::clone(&scope));
        let left_return_type = analyzer.return_type;

        diagnosis.extend(analyzer.diagnosis);

        let analyzer = ExpressionAnalyzer::analyze(&range.right, Rc::clone(&scope));
        let right_return_type = analyzer.return_type;

        diagnosis.extend(analyzer.diagnosis);

        if let TokenKind::DotDot | TokenKind::DotDotEquals = &range.operator.token.kind {
            if left_return_type.is_number() && right_return_type.is_number() {
                return_type = SemanticType::Range;
            } else {
                diagnosis.push(SemanticError::InvalidRangeOperands {
                    left: left_return_type,
                    right: right_return_type,
                    position: range.operator.get_position(),
                })
            }
        }

        Self {
            return_type,
            diagnosis,
        }
    }
}
