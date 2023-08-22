use std::fmt::{Display, Formatter, Result};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Kind {
    BadToken,
    EndOfFileToken,
    NumberToken,
    IdentifierToken,

    // Keywords
    TrueToken,
    FalseToken,
    LetToken,

    // Operators
    PlusToken,
    MinusToken,
    StarToken,
    SlashToken,
    ModToken,

    // Logical
    LogicalNotToken,
    LogicalAndToken,
    LogicalOrToken,
    LogicalEquals,
    LogicalNotEquals,
    LogicalGreaterThan,
    LogicalGreaterThanOrEquals,
    LogicalLessThan,
    LogicalLessThanOrEquals,

    // Bitwise
    BitwiseNotToken,
    BitwiseAndToken,
    BitwiseXorToken,
    BitwiseOrToken,

    WhiteSpaceToken,
    OpenParenthesisToken,
    CloseParenthesisToken,
    OpenBracesToken,
    CloseBracesToken,
    SemicolonToken,
    ColonToken,

    // Expressions
    BinaryExpression,
    UnaryExpression,
    ParenthesizedExpression,

    EqualsToken,

    // Statements
    BlockStatement,
    VariableAssignment,
}

impl Display for Kind {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{:?}", self)
    }
}
