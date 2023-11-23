use std::fmt::{self, Display, Formatter};

use crate::lang::{lexer::token_kind::TokenKind, position::Position};

#[derive(Debug, Clone)]
pub enum SyntaxError {
    TopLevelStatementExpected {
        position: Position,
    },
    UnexpectedToken {
        expected: Vec<TokenKind>,
        found: TokenKind,
        position: Position,
    },
    ExpressionExpected {
        position: Position,
    },
    StatementExpected {
        position: Position,
    },
    AssignmentExpected {
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
            SyntaxError::UnexpectedToken {
                expected,
                found,
                position,
            } => {
                let expected_str: String = expected
                    .iter()
                    .map(|&kind| format!("{}", kind))
                    .collect::<Vec<String>>()
                    .join(", ");

                write!(
                    f,
                    "Expected {} but found {} at Line {} and Column {}",
                    expected_str, found, position.line, position.column
                )
            }
            SyntaxError::ExpressionExpected { position } => {
                write!(
                    f,
                    "Expression expected at Line {} and Column {}",
                    position.line, position.column
                )
            }
            SyntaxError::AssignmentExpected { position } => {
                write!(
                    f,
                    "Assignment expected at Line {} and Column {}",
                    position.line, position.column
                )
            }
            SyntaxError::StatementExpected { position } => {
                write!(
                    f,
                    "Statement expected at Line {} and Column {}",
                    position.line, position.column
                )
            }
        }
    }
}
