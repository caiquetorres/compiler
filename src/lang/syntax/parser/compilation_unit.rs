use crate::lang::syntax::tree_display::TreeDisplay;

use super::statements::TopLevelStatement;

pub struct CompilationUnit(pub Vec<TopLevelStatement>);

impl TreeDisplay for CompilationUnit {
    fn display(&self, layer: usize) {
        let statements = &self.0;
        for statement in statements {
            statement.display(layer);
        }
    }
}
