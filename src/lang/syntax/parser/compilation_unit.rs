use crate::lang::syntax::tree_display::TreeDisplay;

use super::top_level_statements::top_level_statement::TopLevelStatement;

pub struct CompilationUnit {
    pub statements: Vec<TopLevelStatement>,
}

impl CompilationUnit {
    pub fn new(statements: Vec<TopLevelStatement>) -> Self {
        Self { statements }
    }
}

impl TreeDisplay for CompilationUnit {
    fn display(&self, layer: usize) {
        let statements = &self.statements;
        for statement in statements {
            statement.display(layer);
        }
    }
}
