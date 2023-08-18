use std::{collections::VecDeque, fmt::Display};

use super::token::Token;

pub trait Node: ToString {
    fn as_node(&self) -> &dyn Node;

    fn get_token(&self) -> &Token;

    fn get_children(&self) -> Vec<&dyn Node>;
}

impl Display for dyn Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut queue: VecDeque<&dyn Node> = VecDeque::new();
        let mut depth_queue: VecDeque<usize> = VecDeque::new();

        depth_queue.push_back(0);
        queue.push_back(self);

        while !queue.is_empty() {
            let node = queue.pop_back().unwrap();
            let depth = depth_queue.pop_back().unwrap();

            // TODO: We need to improve this visualization. A suggestion would be https://pt.wikipedia.org/wiki/Tree
            write!(f, "{}{}\n", "  ".repeat(depth), node.to_string())?;

            for child in node.get_children() {
                queue.push_back(child);
                depth_queue.push_back(depth + 1);
            }
        }

        Ok(())
    }
}
