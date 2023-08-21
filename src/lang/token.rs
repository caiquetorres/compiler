use super::kind::Kind;
use std::fmt::{Display, Formatter, Result};

#[derive(Debug, Clone)]
pub struct Token {
    pub kind: Kind,
    pub text: String, // REVIEW: Should we make the text optional?
}

impl Display for Token {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{:?}", self)
    }
}

impl Token {
    pub fn new(kind: Kind, text: &str) -> Self {
        Self {
            kind,
            text: String::from(text),
        }
    }
}
