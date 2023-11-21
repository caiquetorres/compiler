use crate::lang::syntax::{
    parser::{expressions::expression::Expression, shared::block::Block},
    tree_display::TreeDisplay,
};

#[derive(Clone)]
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
        println!("{}ElseStatement", " ".repeat(layer));
        self.block.display(layer + 2)
    }
}

#[derive(Clone)]
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
        println!("{}IfStatement", " ".repeat(layer));
        self.block.display(layer + 2);
        self.expression.display(layer + 2);

        if let Some(r#else) = &self.r#else {
            println!("{}ElseStatement", " ".repeat(layer));
            r#else.block.display(layer + 2)
        }
    }
}
