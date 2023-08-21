use super::{node::Node, token::Token};

pub struct ParenthesisNode {
    pub token: Token,
}

impl ParenthesisNode {
    pub fn new(token: Token) -> Self {
        Self { token }
    }
}

impl ToString for ParenthesisNode {
    fn to_string(&self) -> String {
        match &self.token.text[..] {
            "(" => "OpenParenthesisNode".to_string(),
            ")" => "CloseParenthesisNode".to_string(),
            _ => "".to_string(),
        }
    }
}

impl Node for ParenthesisNode {
    fn get_token(&self) -> &Token {
        &self.token
    }

    fn get_children(&self) -> Vec<&dyn Node> {
        vec![]
    }
}
