use super::statement::Statement;
use crate::lang::syntax::{parser::expressions::expression::Expression, tree_display::TreeDisplay};

#[derive(Clone)]
pub struct Else {
    pub statement: Box<Statement>,
}

impl Else {
    pub fn new(statement: Statement) -> Self {
        Self {
            statement: Box::new(statement),
        }
    }
}

impl TreeDisplay for Else {
    fn display(&self, layer: usize) {
        println!("{}ElseStatement", " ".repeat(layer));
        self.statement.display(layer + 2)
    }
}

#[derive(Clone)]
pub struct If {
    pub expression: Expression,
    pub statement: Box<Statement>,
    pub r#else: Option<Else>,
}

impl If {
    pub fn new(expression: Expression, statement: Statement, r#else: Option<Else>) -> Self {
        Self {
            expression,
            statement: Box::new(statement),
            r#else,
        }
    }
}

impl TreeDisplay for If {
    fn display(&self, layer: usize) {
        println!("{}IfStatement", " ".repeat(layer));
        self.statement.display(layer + 2);
        self.expression.display(layer + 2);

        if let Some(r#else) = &self.r#else {
            println!("{}ElseStatement", " ".repeat(layer));
            r#else.statement.display(layer + 2)
        }
    }
}
