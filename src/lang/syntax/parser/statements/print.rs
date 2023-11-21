use crate::lang::syntax::{parser::expressions::expression::Expression, tree_display::TreeDisplay};

#[derive(Clone)]
pub struct Print {
    pub new_line: bool,
    pub expressions: Vec<Expression>,
}

impl Print {
    pub fn new(new_line: bool, expressions: Vec<Expression>) -> Self {
        Self {
            new_line,
            expressions,
        }
    }
}

impl TreeDisplay for Print {
    fn display(&self, layer: usize) {
        println!("{}PrintStatement", " ".repeat(layer));

        for expression in &self.expressions {
            expression.display(layer + 2)
        }
    }
}
