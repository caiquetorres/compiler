use super::{
    node::{ExpressionNode, Node},
    token::Token,
};

#[derive(Debug)]
pub struct NumberExpressionNode {
    pub token: Token,
}

impl NumberExpressionNode {
    pub fn new(token: Token) -> Self {
        Self { token }
    }
}

impl ToString for NumberExpressionNode {
    fn to_string(&self) -> String {
        format!("NumberExpressionNode {{ number: {} }}", self.token.text)
    }
}

impl Node for NumberExpressionNode {
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

impl ExpressionNode for NumberExpressionNode {}
