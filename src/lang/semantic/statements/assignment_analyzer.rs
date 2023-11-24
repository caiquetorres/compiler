use crate::lang::lexer::token_kind::TokenKind;
use crate::lang::semantic::expressions::expression_analyzer::ExpressionAnalyzer;
use crate::lang::semantic::scope::Scope;
use crate::lang::semantic::semantic_error::SemanticError;
use crate::lang::syntax::parser::statements::assignment::Assignment;

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
            diagnosis.push(SemanticError::ValueCannotBeReassigned);
        }

        let right_analyzer = ExpressionAnalyzer::analyze(&assignment.right, Rc::clone(&scope));

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
                diagnosis.push(SemanticError::InvalidLeftOperand)
            } else if !right_analyzer.return_type.is_number() {
                diagnosis.push(SemanticError::InvalidRightOperand)
            }
        }

        if left_analyzer.return_type != right_analyzer.return_type
            && (!left_analyzer.return_type.is_number() || !right_analyzer.return_type.is_number())
        {
            diagnosis.push(SemanticError::TypeMismatch)
        }

        Self { diagnosis }
    }
}
