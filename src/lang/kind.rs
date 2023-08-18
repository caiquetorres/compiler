use std::fmt::{Display, Formatter, Result};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Kind {
    BadToken,
    EndOfFileToken,
    NumberToken,
    PlusToken,
    MinusToken,
    StarToken,
    SlashToken,
    ModToken,
    WhiteSpaceToken,
    OpenParenthesisToken,
    CloseParenthesisToken,
    SemicolonToken,
    BinaryExpression,
    ParenthesizedExpression,
}

impl Display for Kind {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{:?}", self)
    }
}
