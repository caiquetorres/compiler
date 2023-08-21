use super::{node::Node, token::Token};

pub struct LetNode {
    pub token: Token,
}

impl LetNode {
    pub fn new(token: Token) -> Self {
        Self { token }
    }
}

impl ToString for LetNode {
    fn to_string(&self) -> String {
        "LetNode".to_string()
    }
}

impl Node for LetNode {
    fn get_token(&self) -> &Token {
        &self.token
    }

    fn get_children(&self) -> Vec<&dyn Node> {
        vec![]
    }
}
