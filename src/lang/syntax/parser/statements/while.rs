use crate::lang::syntax::{
    parser::{expressions::expression::Expression, shared::block::Block},
    tree_display::TreeDisplay,
};

#[derive(Clone)]
pub struct While {
    pub expression: Expression,
    pub block: Block,
}

impl While {
    pub fn new(expression: Expression, block: Block) -> Self {
        Self { expression, block }
    }
}

impl TreeDisplay for While {
    fn display(&self, layer: usize) {
        println!("{}WhileStatement", " ".repeat(layer));
        self.expression.display(layer + 2);
        self.block.display(layer + 2);
    }
}
