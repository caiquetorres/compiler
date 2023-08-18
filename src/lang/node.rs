use std::{collections::VecDeque, fmt::Display};

pub trait Node: ToString {
    fn get_children(&self) -> Vec<&dyn Node>;
}

// REVIEW: Should we move this for the Tree struct?
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

            // REVIEW: Is this cloning the entire array and its elements?
            for child in node.get_children().iter().rev().cloned() {
                queue.push_back(child);
                depth_queue.push_back(depth + 1);
            }
        }

        Ok(())
    }
}
