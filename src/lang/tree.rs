use std::fmt::{Display, Formatter};

use super::node::Node;

pub struct Tree {
    root: Box<dyn Node>,
}

impl Tree {
    pub fn new(root: Box<dyn Node>) -> Self {
        Self { root }
    }
}

impl Display for Tree {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:}", self.root)
    }
}
