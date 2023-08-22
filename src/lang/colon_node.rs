use super::{node::Node, token::Token};

pub struct ColonNode {
    pub token: Token,
}

impl ColonNode {
    pub fn new(token: Token) -> Self {
        Self { token }
    }
}

impl ToString for ColonNode {
    fn to_string(&self) -> String {
        String::from("ColonNode")
    }
}

impl Node for ColonNode {
    fn get_token(&self) -> &Token {
        &self.token
    }

    fn get_children(&self) -> Vec<&dyn Node> {
        vec![]
    }
}
