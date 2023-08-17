use super::{
    kind::Kind,
    node::{ExpressionNode, Node},
    operator_node::OperatorNode,
    token::Token,
};

pub struct BinaryExpressionNode {
    token: Token,
    pub left: Box<dyn ExpressionNode>,
    pub operator: Box<OperatorNode>,
    pub right: Box<dyn ExpressionNode>,
}

impl BinaryExpressionNode {
    pub fn new(
        left: Box<dyn ExpressionNode>,
        operator: Box<OperatorNode>,
        right: Box<dyn ExpressionNode>,
    ) -> Self {
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
    fn as_node(&self) -> &dyn Node {
        return self;
    }

    fn get_children(&self) -> Vec<&dyn Node> {
        vec![
            self.left.as_ref().as_node(),
            self.operator.as_ref(),
            self.right.as_ref().as_node(),
        ]
    }

    fn get_token(&self) -> &Token {
        &self.token
    }
}

impl ExpressionNode for BinaryExpressionNode {}
