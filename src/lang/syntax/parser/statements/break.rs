use crate::lang::syntax::tree_display::TreeDisplay;

pub struct Break;

impl TreeDisplay for Break {
    fn display(&self, layer: usize) {
        println!("{}BreakStatement", " ".repeat(layer));
    }
}
