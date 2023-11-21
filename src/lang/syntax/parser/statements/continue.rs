use crate::lang::syntax::tree_display::TreeDisplay;

#[derive(Clone)]
pub struct Continue;

impl TreeDisplay for Continue {
    fn display(&self, layer: usize) {
        println!("{}ContinueStatement", " ".repeat(layer));
    }
}
