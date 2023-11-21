use crate::lang::syntax::{parser::expressions::expression::Expression, tree_display::TreeDisplay};

#[derive(Clone)]
pub struct Return {
    pub expression: Option<Expression>,
}

impl Return {
    pub fn new(expression: Option<Expression>) -> Self {
        Self { expression }
    }
}

impl TreeDisplay for Return {
    fn display(&self, layer: usize) {
        println!("{}ReturnStatement", " ".repeat(layer));
        if let Some(ex) = &self.expression {
            ex.display(layer + 2)
        }
    }
}
