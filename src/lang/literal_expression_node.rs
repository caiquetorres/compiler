use super::{node::Node, token::Token};

//REVIEW: Should we rename to just LiteralNode?

#[derive(Debug)]
pub struct LiteralExpressionNode {
    pub token: Token,
}

impl LiteralExpressionNode {
    pub fn new(token: Token) -> Self {
        Self { token }
    }
}

impl ToString for LiteralExpressionNode {
    fn to_string(&self) -> String {
        format!("LiteralExpressionNode {{ number: {} }}", self.token.text)
    }
}

impl Node for LiteralExpressionNode {
    fn get_token(&self) -> &Token {
        &self.token
    }

    fn get_children(&self) -> Vec<&dyn Node> {
        vec![]
    }
}
