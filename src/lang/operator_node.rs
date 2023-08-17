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
    fn as_node(&self) -> &dyn Node {
        return self;
    }

    fn get_children(&self) -> Vec<&dyn Node> {
        vec![]
    }

    fn get_token(&self) -> &Token {
        &self.token
    }
}
