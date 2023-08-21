use super::{kind::Kind, node::Node, parenthesis_node::ParenthesisNode, token::Token};

pub struct ParenthesizedExpressionNode {
    pub token: Token,
    pub open_parenthesis: Box<ParenthesisNode>,
    pub expression: Box<dyn Node>,
    pub close_parenthesis: Box<ParenthesisNode>,
}

impl ParenthesizedExpressionNode {
    pub fn new(
        open_parenthesis: Box<ParenthesisNode>,
        expression: Box<dyn Node>,
        close_parenthesis: Box<ParenthesisNode>,
    ) -> Self {
        Self {
            token: Token::new(Kind::ParenthesizedExpression, ""),
            open_parenthesis,
            expression,
            close_parenthesis,
        }
    }
}

impl ToString for ParenthesizedExpressionNode {
    fn to_string(&self) -> String {
        "ParenthesizedExpressionNode".to_string()
    }
}

impl Node for ParenthesizedExpressionNode {
    fn get_token(&self) -> &Token {
        &self.token
    }

    fn get_children(&self) -> Vec<&dyn Node> {
        vec![self.expression.as_ref()]
    }
}
