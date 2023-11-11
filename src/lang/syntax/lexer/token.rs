use super::kind::Kind;

#[derive(Clone, Copy)]
pub struct Position {
    pub position: usize,
    pub column: usize,
    pub line: usize,
}

impl Position {
    pub fn new(position: usize, column: usize, line: usize) -> Self {
        Self {
            position,
            column,
            line,
        }
    }
}

pub struct Token {
    pub kind: Kind,
    pub position: Position,
    pub value: Option<String>,
}

impl Token {
    pub fn new(kind: Kind, position: Position, value: Option<&str>) -> Self {
        Self {
            kind,
            position,
            value: value.map(|s| s.to_string()),
        }
    }
}
