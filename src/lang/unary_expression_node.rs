use super::{kind::Kind, node::Node, operator_node::OperatorNode, token::Token};

pub struct UnaryExpressionNode {
    pub token: Token,
    pub operator: Box<OperatorNode>,
    pub child: Box<dyn Node>,
}

impl UnaryExpressionNode {
    pub fn new(operator: Box<OperatorNode>, child: Box<dyn Node>) -> Self {
        Self {
            operator,
            child,
            token: Token::new(Kind::UnaryExpression, ""),
        }
    }
}

impl ToString for UnaryExpressionNode {
    fn to_string(&self) -> String {
        "UnaryExpressionNode".to_string()
    }
}

impl Node for UnaryExpressionNode {
    fn get_token(&self) -> &Token {
        &self.token
    }

    fn get_children(&self) -> Vec<&dyn Node> {
        vec![self.operator.as_ref(), self.child.as_ref()]
    }
}
