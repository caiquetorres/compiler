use std::{cell::RefCell, rc::Rc};

use crate::lang::syntax::{
    lexer::token_kind::TokenKind, parser::statements::assignment::Assignment,
};

use super::{
    expression_analyzer::ExpressionAnalyzer, lang_type::LangType, scope::Scope,
    semantic_error::SemanticError, symbol::Symbol,
};

pub struct AssignmentAnalyzer {
    pub(crate) diagnosis: Vec<SemanticError>,
}

impl AssignmentAnalyzer {
    pub fn analyze(assignment: &Assignment, scope: Rc<RefCell<Scope>>) -> Self {
        let mut diagnosis: Vec<SemanticError> = vec![];

        let identifier_name = assignment.identifier.name.clone();
        let mut identifier_type = LangType::Any;

        if let Some(symbol) = scope.borrow().get(&identifier_name) {
            if let Symbol::Variable { symbol_type, .. } = symbol {
                identifier_type = symbol_type;
            } else {
                diagnosis.push(SemanticError::ValueCannotBeReassigned);
            }
        } else {
            diagnosis.push(SemanticError::IdentifierNotFound);
        }

        let analyzer = ExpressionAnalyzer::analyze(&assignment.expression, Rc::clone(&scope));

        if let TokenKind::PlusEquals
        | TokenKind::MinusEquals
        | TokenKind::StarEquals
        | TokenKind::SlashEquals
        | TokenKind::ModEquals
        | TokenKind::AmpersandEquals
        | TokenKind::PipeEquals
        | TokenKind::CircumflexEquals = &assignment.operator.token.kind
        {
            if !identifier_type.is_number() {
                diagnosis.push(SemanticError::InvalidLeftOperand)
            } else if !analyzer.return_type.is_number() {
                diagnosis.push(SemanticError::InvalidRightOperand)
            }
        }

        if (!identifier_type.is_number() || !analyzer.return_type.is_number())
            && identifier_type != analyzer.return_type
        {
            diagnosis.push(SemanticError::TypeMismatch)
        }

        Self { diagnosis }
    }
}
