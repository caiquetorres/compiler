use std::fmt::{Display, Formatter, Result};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Kind {
    BadToken,
    EndOfFileToken,
    NumberToken,
    TrueToken,
    FalseToken,
    NameToken,

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
    SemicolonToken,

    // Expressions
    BinaryExpression,
    UnaryExpression,
    ParenthesizedExpression,
}

impl Display for Kind {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{:?}", self)
    }
}
