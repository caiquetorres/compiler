use crate::lang::syntax::tree_display::TreeDisplay;

#[derive(Clone, Debug)]
pub struct Continue;

impl TreeDisplay for Continue {
    fn display(&self, layer: usize) {
        println!("{}ContinueStatement", " ".repeat(layer));
    }
}
