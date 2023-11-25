use crate::lang::syntax::{
    expressions::expression::Expression,
    shared::{block::Block, identifier::Identifier},
    tree_display::TreeDisplay,
};

#[derive(Clone, Debug)]
pub struct For {
    pub identifier: Identifier,
    pub expression: Expression,
    pub block: Block,
}

impl For {
    pub fn new(identifier: Identifier, expression: Expression, block: Block) -> Self {
        Self {
            identifier,
            expression,
            block,
        }
    }
}

impl TreeDisplay for For {
    fn display(&self, layer: usize) {
        println!(
            "{}ForStatement ({})",
            "  ".repeat(layer),
            self.identifier.name
        );

        self.expression.display(layer + 1);
        self.block.display(layer + 1);
    }
}
