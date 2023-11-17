#[derive(Clone, Copy, Debug)]
pub struct Position {
    pub line: usize,
    pub column: usize,
}

impl Position {
    pub fn new(column: usize, line: usize) -> Self {
        Self { column, line }
    }
}
