use crate::lang::syntax::{
    expressions::expression::Expression, shared::block::Block, tree_display::TreeDisplay,
};

#[derive(Clone, Debug)]
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
        println!("{}WhileStatement", "  ".repeat(layer));
        self.expression.display(layer + 1);
        self.block.display(layer + 1);
    }
}
