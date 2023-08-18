use super::{node::Node, token::Token};

#[derive(Debug)]
pub struct OperatorNode {
    pub token: Token,
}

impl OperatorNode {
    pub fn new(token: Token) -> Self {
        Self { token }
    }
}

impl ToString for OperatorNode {
    fn to_string(&self) -> String {
        format!("OperatorNode {{ operator: {:?} }}", self.token.text)
    }
}

impl Node for OperatorNode {
    fn get_children(&self) -> Vec<&dyn Node> {
        vec![]
    }
}
