use crate::lang::{
    position::{Position, Positioned},
    syntax::{expressions::expression::Expression, tree_display::TreeDisplay},
};

#[derive(Clone, Debug)]
pub struct Return {
    position: Position,
    pub expression: Option<Expression>,
}

impl Return {
    pub fn new(expression: Option<Expression>, position: Position) -> Self {
        Self {
            expression,
            position,
        }
    }
}

impl Positioned for Return {
    fn get_position(&self) -> Position {
        self.position
    }
}

impl TreeDisplay for Return {
    fn display(&self, layer: usize) {
        println!("{}ReturnStatement", "  ".repeat(layer));
        if let Some(ex) = &self.expression {
            ex.display(layer + 1)
        }
    }
}
