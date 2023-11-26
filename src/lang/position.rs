#[derive(Clone, Copy, Debug)]
pub struct Position {
    pub line: usize,
    pub column: usize,
}

pub trait Positioned {
    fn get_position(&self) -> Position;
}

impl Position {
    pub fn new(column: usize, line: usize) -> Self {
        Self { column, line }
    }
}
