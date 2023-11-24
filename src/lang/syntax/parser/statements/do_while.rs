use crate::lang::syntax::{
    parser::{expressions::expression::Expression, shared::block::Block},
    tree_display::TreeDisplay,
};

#[derive(Clone, Debug)]
pub struct DoWhile {
    pub block: Block,
    pub expression: Expression,
}

impl DoWhile {
    pub fn new(block: Block, expression: Expression) -> Self {
        Self { block, expression }
    }
}

impl TreeDisplay for DoWhile {
    fn display(&self, layer: usize) {
        println!("{}DoWhileStatement", "  ".repeat(layer));
        self.block.display(layer + 1);
        self.expression.display(layer + 1);
    }
}
