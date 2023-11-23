use std::cell::RefCell;
use std::rc::Rc;

use crate::lang::lexer::token_kind::TokenKind;
use crate::lang::semantic::expression_analyzer::ExpressionAnalyzer;
use crate::lang::semantic::scope::Scope;
use crate::lang::semantic::semantic_error::SemanticError;
use crate::lang::semantic::semantic_type::SemanticType;
use crate::lang::syntax::parser::expressions::binary::Binary;

pub struct BinaryAnalyzer {
    pub return_type: SemanticType,
    pub diagnosis: Vec<SemanticError>,
}

impl BinaryAnalyzer {
    pub fn analyze(binary: &Binary, scope: Rc<RefCell<Scope>>) -> Self {
        let mut return_type = SemanticType::Any;
        let mut diagnosis: Vec<SemanticError> = vec![];

        let analyzer = ExpressionAnalyzer::analyze(&binary.left, Rc::clone(&scope));
        let left_return_type = analyzer.return_type;

        diagnosis.extend(analyzer.diagnosis);

        let analyzer = ExpressionAnalyzer::analyze(&binary.right, Rc::clone(&scope));
        let right_return_type = analyzer.return_type;

        diagnosis.extend(analyzer.diagnosis);

        match &binary.operator.token.kind {
            TokenKind::EqualsEquals | TokenKind::ExclamationEquals => {
                if left_return_type.is_number() && right_return_type.is_number() {
                    return_type = SemanticType::Bool;
                } else if left_return_type == right_return_type {
                    return_type = SemanticType::Bool;
                } else {
                    diagnosis.push(SemanticError::EqualityTypeMismatch)
                }
            }
            TokenKind::Plus | TokenKind::Minus | TokenKind::Star | TokenKind::Slash => {
                if left_return_type.is_number() && right_return_type.is_number() {
                    return_type = SemanticType::number_type_precedence(vec![
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
                    return_type = SemanticType::number_type_precedence(vec![
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
                    return_type = SemanticType::Bool;
                } else {
                    diagnosis.push(SemanticError::InvalidOperator)
                }
            }
            TokenKind::AmpersandAmpersand | TokenKind::PipePipe => {
                if left_return_type == SemanticType::Bool && right_return_type == SemanticType::Bool
                {
                    return_type = SemanticType::Bool;
                } else {
                    diagnosis.push(SemanticError::InvalidOperator)
                }
            }
            _ => unreachable!(),
        }

        Self {
            return_type,
            diagnosis,
        }
    }
}
