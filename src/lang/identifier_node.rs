use super::{node::Node, token::Token};

pub struct IdentifierNode {
    pub token: Token,
}

impl IdentifierNode {
    pub fn new(token: Token) -> Self {
        Self { token }
    }
}

impl ToString for IdentifierNode {
    fn to_string(&self) -> String {
        format!("IdentifierNode {{ name: {:} }}", self.token.text)
    }
}

impl Node for IdentifierNode {
    fn get_token(&self) -> &Token {
        &self.token
    }

    fn get_children(&self) -> Vec<&dyn Node> {
        vec![]
    }
}
