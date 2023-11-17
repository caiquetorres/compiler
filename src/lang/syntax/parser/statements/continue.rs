use crate::lang::syntax::tree_display::TreeDisplay;

pub struct Continue;

impl TreeDisplay for Continue {
    fn display(&self, layer: usize) {
        println!("{}ContinueStatement", " ".repeat(layer));
    }
}
