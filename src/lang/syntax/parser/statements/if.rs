use crate::lang::syntax::{
    parser::{expressions::expression::Expression, shared::block::Block},
    tree_display::TreeDisplay,
};

#[derive(Clone, Debug)]
pub struct Else {
    pub block: Block,
}

impl Else {
    pub fn new(block: Block) -> Self {
        Self { block }
    }
}

impl TreeDisplay for Else {
    fn display(&self, layer: usize) {
        println!("{}ElseStatement", "  ".repeat(layer));
        self.block.display(layer + 1)
    }
}

#[derive(Clone, Debug)]
pub struct If {
    pub expression: Expression,
    pub block: Block,
    pub r#else: Option<Else>,
}

impl If {
    pub fn new(expression: Expression, block: Block, r#else: Option<Else>) -> Self {
        Self {
            expression,
            block,
            r#else,
        }
    }
}

impl TreeDisplay for If {
    fn display(&self, layer: usize) {
        println!("{}IfStatement", "  ".repeat(layer));
        self.expression.display(layer + 1);
        self.block.display(layer + 1);

        if let Some(r#else) = &self.r#else {
            println!("{}ElseStatement", "  ".repeat(layer));
            r#else.block.display(layer + 1)
        }
    }
}
