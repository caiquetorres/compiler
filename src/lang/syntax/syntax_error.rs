use std::fmt::{self, Display, Formatter};

use crate::lang::{lexer::token_kind::TokenKind, position::Position};

#[derive(Debug, Clone)]
pub enum SyntaxError {
    UnexpectedToken {
        found: TokenKind,
        position: Position,
    },
    TopLevelStatementExpected {
        position: Position,
    },
    ExpressionExpected {
        position: Position,
    },
}

impl Display for SyntaxError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            SyntaxError::TopLevelStatementExpected { position } => {
                write!(
                    f,
                    "Top-level statement expected at Line {} and Column {}",
                    position.line, position.column
                )
            }
            SyntaxError::UnexpectedToken { found, position } => {
                write!(
                    f,
                    "Unexpected {} at Line {} and Column {}",
                    found, position.line, position.column
                )
            }
            SyntaxError::ExpressionExpected { position } => {
                write!(
                    f,
                    "Expression expected at Line {} and Column {}",
                    position.line, position.column
                )
            }
        }
    }
}
