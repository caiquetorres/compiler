use crate::lang::syntax::{parser::expressions::expression::Expression, tree_display::TreeDisplay};

pub struct Return(pub Option<Expression>);

impl TreeDisplay for Return {
    fn display(&self, layer: usize) {
        println!("{}ReturnStatement", " ".repeat(layer));
        if let Some(ex) = &self.0 {
            ex.display(layer + 2)
        }
    }
}
