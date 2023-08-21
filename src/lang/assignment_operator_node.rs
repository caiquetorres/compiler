use super::{node::Node, token::Token};

pub struct AssignmentOperatorNode {
    pub token: Token,
}

impl AssignmentOperatorNode {
    pub fn new(token: Token) -> Self {
        Self { token }
    }
}

impl ToString for AssignmentOperatorNode {
    fn to_string(&self) -> String {
        format!("AssignmentOperatorNode {{ operator {:} }}", self.token.text)
    }
}

impl Node for AssignmentOperatorNode {
    fn get_token(&self) -> &Token {
        &self.token
    }

    fn get_children(&self) -> Vec<&dyn Node> {
        vec![]
    }
}
