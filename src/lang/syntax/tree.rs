use std::fmt::{Display, Formatter};

use super::{expressions::TreeDisplay, statements::Statement};

pub struct Tree {
    root: Statement,
}

impl Tree {
    pub fn new(root: Statement) -> Self {
        Self { root }
    }
}

impl Display for Tree {
    fn fmt(&self, _f: &mut Formatter<'_>) -> std::fmt::Result {
        Ok(self.root.display(0))
    }
}
