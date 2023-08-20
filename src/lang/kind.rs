use std::fmt::{Display, Formatter, Result};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Kind {
    BadToken,
    EndOfFileToken,
    NumberToken,

    // Operators
    PlusToken,
    MinusToken,
    StarToken,
    SlashToken,
    ModToken,
    // LogicalAnd, LogicalOr, Relational, (<=, <, >, >=) Equality (==, !=)
    LogicalNotToken,
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
    TrueToken,
    FalseToken,
}

impl Display for Kind {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{:?}", self)
    }
}
