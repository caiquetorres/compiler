use crate::lang::{
    position::{Position, Positioned},
    syntax::tree_display::TreeDisplay,
};

#[derive(Clone, Debug)]
pub struct Break {
    position: Position,
}

impl Break {
    pub fn new(position: Position) -> Self {
        Self { position }
    }
}

impl Positioned for Break {
    fn get_position(&self) -> crate::lang::position::Position {
        self.position
    }
}

impl TreeDisplay for Break {
    fn display(&self, layer: usize) {
        println!("{}BreakStatement", "  ".repeat(layer));
    }
}
