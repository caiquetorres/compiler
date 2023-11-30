use crate::lang::semantic::expressions::expression_analyzer::ExpressionAnalyzer;
use crate::lang::semantic::scope::Scope;
use crate::lang::semantic::semantic_error::SemanticError;
use crate::lang::syntax::statements::assignment::Assignment;
use crate::lang::{lexer::token_kind::TokenKind, position::Positioned};

use std::{cell::RefCell, rc::Rc};

pub struct AssignmentAnalyzer {
    pub(crate) diagnosis: Vec<SemanticError>,
}

impl AssignmentAnalyzer {
    pub fn analyze(assignment: &Assignment, scope: Rc<RefCell<Scope>>) -> Self {
        let mut diagnosis: Vec<SemanticError> = vec![];

        let left_analyzer = ExpressionAnalyzer::analyze(&assignment.left, Rc::clone(&scope));
        diagnosis.extend(left_analyzer.diagnosis);

        if !left_analyzer.changeable {
            diagnosis.push(SemanticError::ValueCannotBeReassigned {
                position: assignment.left.get_position(),
            });
        }

        let right_analyzer = ExpressionAnalyzer::analyze(&assignment.right, Rc::clone(&scope));
        diagnosis.extend(right_analyzer.diagnosis);

        if let TokenKind::PlusEquals
        | TokenKind::MinusEquals
        | TokenKind::StarEquals
        | TokenKind::SlashEquals
        | TokenKind::ModEquals
        | TokenKind::AmpersandEquals
        | TokenKind::PipeEquals
        | TokenKind::CircumflexEquals = &assignment.operator.token.kind
        {
            if !left_analyzer.return_type.is_number() {
                diagnosis.push(SemanticError::InvalidLeftOperand {
                    position: assignment.left.get_position(),
                })
            } else if !right_analyzer.return_type.is_number() {
                diagnosis.push(SemanticError::InvalidRightOperand {
                    position: assignment.right.get_position(),
                })
            }
        }

        if left_analyzer.return_type != right_analyzer.return_type
            && (!left_analyzer.return_type.is_number() || !right_analyzer.return_type.is_number())
        {
            diagnosis.push(SemanticError::TypeMismatch {
                left: left_analyzer.return_type,
                right: right_analyzer.return_type,
                position: assignment.operator.get_position(),
            })
        }

        Self { diagnosis }
    }
}
