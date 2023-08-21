use super::{node::Node, token::Token};

pub struct SemicolonNode {
    pub token: Token,
}

impl SemicolonNode {
    pub fn new(token: Token) -> Self {
        Self { token }
    }
}

impl ToString for SemicolonNode {
    fn to_string(&self) -> String {
        String::from("SemicolonNode")
    }
}

impl Node for SemicolonNode {
    fn get_token(&self) -> &Token {
        &self.token
    }

    fn get_children(&self) -> Vec<&dyn Node> {
        vec![]
    }
}
