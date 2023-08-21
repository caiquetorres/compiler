use super::{node::Node, token::Token};

pub struct BraceNode {
    pub token: Token,
}

impl BraceNode {
    pub fn new(token: Token) -> Self {
        Self { token }
    }
}

impl ToString for BraceNode {
    fn to_string(&self) -> String {
        match &self.token.text[..] {
            "(" => "OpenBraceNode".to_string(),
            ")" => "CloseBraceNode".to_string(),
            _ => "".to_string(),
        }
    }
}

impl Node for BraceNode {
    fn get_token(&self) -> &Token {
        &self.token
    }

    fn get_children(&self) -> Vec<&dyn Node> {
        vec![]
    }
}
