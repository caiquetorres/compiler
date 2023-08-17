use super::{
    binary_expression_node::BinaryExpressionNode, kind::Kind, lexer::Lexer, node::ExpressionNode,
    number_expression_node::NumberExpressionNode, operator_node::OperatorNode, token::Token,
};

pub struct Parser {
    tokens: Vec<Token>,
}

impl Parser {
    pub fn new(text: &str) -> Self {
        let mut tokens: Vec<Token> = vec![];
        let mut lexer = Lexer::new(text);

        let mut token: Token;

        loop {
            token = lexer.next_token();

            if token.kind == Kind::WhiteSpaceToken || token.kind == Kind::BadToken {
                continue;
            }

            tokens.push(token.clone());

            if token.kind == Kind::EndOfFileToken {
                break;
            }
        }

        Self { tokens }
    }

    // REVIEW: Currently the parser is just parsing binary expressions such as "2 + 2"
    pub fn parse(&self) -> Box<dyn ExpressionNode> {
        // REVIEW: Is there any possibility of the text being empty?

        let mut position: usize = 0;
        let mut token = self.tokens[position].clone();

        let left_token = token.clone();
        let mut left: Box<dyn ExpressionNode> = Box::new(NumberExpressionNode::new(left_token));

        position += 1;
        token = self.tokens[position].clone();

        // REVIEW: Should we replace for a match-expression?
        while token.kind == Kind::PlusToken || token.kind == Kind::MinusToken {
            let operator_token = self.tokens[position].clone();
            let operator_node = OperatorNode::new(operator_token);

            position += 1;
            token = self.tokens[position].clone();

            let right_token = token.clone();
            let right = Box::new(NumberExpressionNode::new(right_token));

            left = Box::new(BinaryExpressionNode::new(
                left,
                Box::new(operator_node),
                right,
            ));

            position += 1;
            token = self.tokens[position].clone();
        }

        left
    }
}
