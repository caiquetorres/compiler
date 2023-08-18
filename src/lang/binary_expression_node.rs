use super::{kind::Kind, node::Node, operator_node::OperatorNode, token::Token};

pub struct BinaryExpressionNode {
    pub token: Token,
    pub left: Box<dyn Node>,
    pub operator: Box<OperatorNode>,
    pub right: Box<dyn Node>,
}

impl BinaryExpressionNode {
    pub fn new(left: Box<dyn Node>, operator: Box<OperatorNode>, right: Box<dyn Node>) -> Self {
        Self {
            left,
            operator,
            right,
            token: Token::new(Kind::BinaryExpression, ""),
        }
    }
}

impl ToString for BinaryExpressionNode {
    fn to_string(&self) -> String {
        "BinaryExpressionNode".to_string()
    }
}

impl Node for BinaryExpressionNode {
    fn get_children(&self) -> Vec<&dyn Node> {
        vec![
            self.left.as_ref(),
            self.operator.as_ref(),
            self.right.as_ref(),
        ]
    }
}
