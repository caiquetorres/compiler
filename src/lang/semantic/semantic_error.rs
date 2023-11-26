use std::fmt::{self, Display, Formatter};

use crate::lang::position::Position;

use super::semantic_type::SemanticType;

#[derive(Debug)]
pub enum SemanticError {
    DuplicatedIdentifier {
        position: Position,
    },

    IdentifierNotFound {
        position: Position,
    },

    // fun main(a: i32) { ... }
    MainFunctionWithParameters {
        position: Position,
    },

    // fun main() -> i32 { ... }
    MainFunctionWithReturn {
        position: Position,
    },

    // let x;
    MissingTypeOrExpression {
        position: Position,
    },

    InvalidUnaryOperand {
        found: SemanticType,
        position: Position,
    },

    // true..2
    InvalidRangeOperands {
        left: SemanticType,
        right: SemanticType,
        position: Position,
    },

    // 2 + a
    IdentifierNotVariableOrParam {
        position: Position,
    },

    // a()
    IdentifierNotCallable {
        position: Position,
    },

    // func(1, 2, 3)
    InvalidNumberOfParameters {
        expected: usize,
        found: usize,
        position: Position,
    },

    // func(true)
    InvalidParameterType {
        expected: SemanticType,
        found: SemanticType,
        position: Position,
    },

    // a += true
    TypeMismatch {
        left: SemanticType,
        right: SemanticType,
        position: Position,
    },

    // 2 == true
    EqualityTypeMismatch {
        left: SemanticType,
        right: SemanticType,
        position: Position,
    },

    InvalidOperator {
        left: SemanticType,
        right: SemanticType,
        position: Position,
    }, // 2 + true, 2.0 & 3.0, true >= true, 2 && 2

    ValueCannotBeReassigned {
        position: Position,
    }, // a = 2

    InvalidLeftOperand {
        position: Position,
    }, // a += 2

    InvalidRightOperand {
        position: Position,
    }, // a += true

    InvalidBreak {
        position: Position,
    },

    InvalidContinue {
        position: Position,
    },

    InvalidReturn {
        position: Position,
    },

    IdentifierNotIndexable {
        position: Position,
    },

    CannotReturnArray {
        position: Position,
    },

    CannotReturnFunction {
        position: Position,
    },

    ImmediateArrayUsageWithoutAssignment {
        position: Position,
    },

    ExpectedType {
        expected: SemanticType,
        found: SemanticType,
        position: Position,
    },

    InvalidArrayElement {
        position: Position,
    },
}

impl Display for SemanticError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::DuplicatedIdentifier { position } => write!(
                f,
                "Duplicate identifier found at Line {} and Column {}",
                position.line, position.column
            ),
            Self::IdentifierNotFound { position } => {
                write!(
                    f,
                    "Identifier not found at Line {} and Column {}",
                    position.line, position.column
                )
            }
            Self::MainFunctionWithParameters { position } => {
                write!(
                    f,
                    "Main function cannot have parameters at Line {} and Column {}",
                    position.line, position.column
                )
            }
            Self::MainFunctionWithReturn { position } => {
                write!(
                    f,
                    "Main function cannot have a return value at Line {} and Column {}",
                    position.line, position.column
                )
            }
            Self::MissingTypeOrExpression { position } => {
                write!(
                    f,
                    "Missing type or expression at Line {} and Column {}",
                    position.line, position.column
                )
            }
            Self::InvalidUnaryOperand { found, position } => {
                write!(
                    f,
                    "Invalid unary operand of type '{}' at Line {} and Column {}",
                    found.to_string(),
                    position.line,
                    position.column
                )
            }
            Self::InvalidRangeOperands {
                left,
                right,
                position,
            } => {
                write!(
                    f,
                    "Invalid range operands: '{}' and '{}' at Line {} and Column {}",
                    left.to_string(),
                    right.to_string(),
                    position.line,
                    position.column
                )
            }
            Self::IdentifierNotVariableOrParam { position } => {
                write!(
                    f,
                    "Identifier is not a variable or parameter at Line {} and Column {}",
                    position.line, position.column
                )
            }
            Self::IdentifierNotCallable { position } => {
                write!(
                    f,
                    "Identifier is not callable at Line {} and Column {}",
                    position.line, position.column
                )
            }
            Self::InvalidNumberOfParameters {
                expected,
                found,
                position,
            } => {
                write!(
                    f,
                    "Invalid number of parameters. Expected {} but found {} at Line {} and Column {}",
                    expected, found, position.line, position.column
                )
            }
            Self::InvalidParameterType {
                expected,
                found,
                position,
            } => {
                write!(
                    f,
                    "Invalid parameter type. Expected '{}' but found '{}' at Line {} and Column {}",
                    expected.to_string(),
                    found.to_string(),
                    position.line,
                    position.column
                )
            }
            Self::TypeMismatch {
                left,
                right,
                position,
            } => {
                write!(
                    f,
                    "Type mismatch. Expected '{}' but found '{}' at Line {} and Column {}",
                    left.to_string(),
                    right.to_string(),
                    position.line,
                    position.column
                )
            }
            Self::EqualityTypeMismatch {
                left,
                right,
                position,
            } => {
                write!(
                    f,
                    "Equality type mismatch. Cannot compare types '{}' and '{}' at Line {} and Column {}",
                    left.to_string(), right.to_string(), position.line, position.column
                )
            }
            Self::InvalidOperator {
                left,
                right,
                position,
            } => {
                write!(
                    f,
                    "Invalid operator. Cannot perform operation between types '{}' and '{}' at Line {} and Column {}",
                    left.to_string(), right.to_string(), position.line, position.column
                )
            }
            Self::ValueCannotBeReassigned { position } => {
                write!(
                    f,
                    "Value cannot be reassigned at Line {} and Column {}",
                    position.line, position.column
                )
            }
            Self::InvalidLeftOperand { position } => {
                write!(
                    f,
                    "Invalid left operand at Line {} and Column {}",
                    position.line, position.column
                )
            }
            Self::InvalidRightOperand { position } => {
                write!(
                    f,
                    "Invalid right operand at Line {} and Column {}",
                    position.line, position.column
                )
            }
            Self::InvalidBreak { position } => {
                write!(
                    f,
                    "Invalid 'break' statement at Line {} and Column {}",
                    position.line, position.column
                )
            }
            Self::InvalidContinue { position } => {
                write!(
                    f,
                    "Invalid 'continue' statement at Line {} and Column {}",
                    position.line, position.column
                )
            }
            Self::InvalidReturn { position } => {
                write!(
                    f,
                    "Invalid 'return' statement at Line {} and Column {}",
                    position.line, position.column
                )
            }
            Self::IdentifierNotIndexable { position } => {
                write!(
                    f,
                    "Identifier is not indexable at Line {} and Column {}",
                    position.line, position.column
                )
            }
            Self::CannotReturnArray { position } => {
                write!(
                    f,
                    "Cannot return an array at Line {} and Column {}",
                    position.line, position.column
                )
            }
            Self::CannotReturnFunction { position } => {
                write!(
                    f,
                    "Cannot return a function at Line {} and Column {}",
                    position.line, position.column
                )
            }
            Self::ImmediateArrayUsageWithoutAssignment { position } => {
                write!(
                    f,
                    "Immediate array usage without assignment at Line {} and Column {}",
                    position.line, position.column
                )
            }
            Self::ExpectedType {
                expected,
                found,
                position,
            } => {
                write!(
                    f,
                    "Expected type '{}' but found '{}' at Line {} and Column {}",
                    expected.to_string(),
                    found.to_string(),
                    position.line,
                    position.column
                )
            }
            Self::InvalidArrayElement { position } => {
                write!(
                    f,
                    "Invalid array element at Line {} and Column {}",
                    position.line, position.column
                )
            }
        }
    }
}
