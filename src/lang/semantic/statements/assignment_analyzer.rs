use std::{cell::RefCell, rc::Rc};

use crate::lang::lexer::token_kind::TokenKind;
use crate::lang::semantic::expression_analyzer::ExpressionAnalyzer;
use crate::lang::semantic::scope::Scope;
use crate::lang::semantic::semantic_error::SemanticError;
use crate::lang::semantic::semantic_type::SemanticType;
use crate::lang::semantic::symbol::Symbol;
use crate::lang::syntax::parser::shared::identifier::IdentifierMeta;
use crate::lang::syntax::parser::statements::assignment::Assignment;

pub struct AssignmentAnalyzer {
    pub(crate) diagnosis: Vec<SemanticError>,
}

impl AssignmentAnalyzer {
    pub fn analyze(assignment: &Assignment, scope: Rc<RefCell<Scope>>) -> Self {
        let mut diagnosis: Vec<SemanticError> = vec![];

        let identifier_name = assignment.identifier.name.clone();
        let mut identifier_type = SemanticType::Any;

        if let Some(symbol) = scope.borrow().get(&identifier_name) {
            if let Symbol::Variable { symbol_type, .. } = symbol {
                if let Some(meta) = &assignment.identifier.meta {
                    match meta {
                        IdentifierMeta::Index(expression, meta) => match symbol_type {
                            SemanticType::Array(r#type, ..) => {
                                let analyzer = ExpressionAnalyzer::analyze(
                                    expression.as_ref(),
                                    Rc::clone(&scope),
                                );

                                diagnosis.extend(analyzer.diagnosis);

                                // TODO: Recursive validation here
                                if let None = meta.as_ref() {
                                    identifier_type = r#type.as_ref().clone();
                                }
                            }
                            _ => diagnosis.push(SemanticError::IdentifierNotIndexable),
                        },
                    }
                } else {
                    identifier_type = symbol_type;
                }
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

        if identifier_type != analyzer.return_type
            && (!identifier_type.is_number() || !analyzer.return_type.is_number())
        {
            diagnosis.push(SemanticError::TypeMismatch)
        }

        Self { diagnosis }
    }
}
