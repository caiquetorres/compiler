use crate::lang::syntax::tree_display::TreeDisplay;

use super::function::Function;

#[derive(Clone, Debug)]
pub enum TopLevelStatement {
    Function(Function),
}

impl TreeDisplay for TopLevelStatement {
    fn display(&self, layer: usize) {
        match self {
            Self::Function(function) => function.display(layer),
        }
    }
}
