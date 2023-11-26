use crate::lang::{
    position::{Position, Positioned},
    syntax::tree_display::TreeDisplay,
};

#[derive(Clone, Debug)]
pub struct Continue {
    position: Position,
}

impl Continue {
    pub fn new(position: Position) -> Self {
        Self { position }
    }
}

impl Positioned for Continue {
    fn get_position(&self) -> crate::lang::position::Position {
        self.position
    }
}

impl TreeDisplay for Continue {
    fn display(&self, layer: usize) {
        println!("{}ContinueStatement", "  ".repeat(layer));
    }
}
