use crate::lang::{binary_expression_node::BinaryExpressionNode, operator_node::OperatorNode};

use super::{
    kind::Kind, lexer::Lexer, literal_expression_node::LiteralExpressionNode, node::Node,
    parenthesis_node::ParenthesisNode, parenthesized_expression_node::ParenthesizedExpressionNode,
    token::Token, tree::Tree, unary_expression_node::UnaryExpressionNode,
};

pub struct Parser {
    current_position: isize,
    tokens: Vec<Token>,
}

impl Parser {
    pub fn new(text: &str) -> Self {
        let mut lexer = Lexer::new(text);
        let mut tokens: Vec<Token> = vec![];

        let mut token = lexer.next();
        while token.kind != Kind::EndOfFileToken {
            if token.kind == Kind::WhiteSpaceToken {
                token = lexer.next();
                continue;
            }

            tokens.push(token);
            token = lexer.next();
        }

        tokens.push(Token::new(Kind::EndOfFileToken, ""));

        Self {
            current_position: -1,
            tokens,
        }
    }

    pub fn parse(&mut self) -> Result<Tree, String> {
        let bad_token = self
            .tokens
            .iter()
            .find(|token| token.kind == Kind::BadToken);

        if bad_token.is_some() {
            return Err(format!("Bad token: {}", bad_token.unwrap().text));
        }

        let expression = self.parse_expression(0)?;
        let tree = Tree::new(expression);
        return Ok(tree);
    }

    fn current_token(&self) -> Token {
        match self.tokens.get(self.current_position as usize) {
            Some(token) => token.clone(),
            None => Token::new(Kind::EndOfFileToken, ""),
        }
    }

    fn next_token(&mut self) -> Token {
        self.current_position += 1;
        match self.tokens.get(self.current_position as usize) {
            Some(token) => token.clone(),
            None => Token::new(Kind::EndOfFileToken, ""),
        }
    }

    fn parse_expression(&mut self, parent_precedence: u8) -> Result<Box<dyn Node>, String> {
        let mut left: Box<dyn Node> = self.parse_factor()?;

        let operator_token = self.next_token();
        let mut precedence = get_binary_operator_precedence(operator_token.kind);

        while precedence != 0 && precedence > parent_precedence {
            let operator_token = self.current_token();
            let operator = Box::new(OperatorNode::new(operator_token));
            let right = self.parse_expression(precedence)?;

            left = Box::new(BinaryExpressionNode::new(left, operator, right));
            precedence = get_binary_operator_precedence(self.current_token().kind);
        }

        Ok(left)
    }

    fn parse_factor(&mut self) -> Result<Box<dyn Node>, String> {
        let token = self.next_token();

        match token.kind {
            Kind::NumberToken | Kind::TrueToken | Kind::FalseToken => {
                Ok(Box::new(LiteralExpressionNode::new(token)))
            }
            Kind::PlusToken | Kind::MinusToken => Ok(Box::new(UnaryExpressionNode::new(
                Box::new(OperatorNode::new(self.current_token())),
                self.parse_factor()?,
            ))),
            Kind::OpenParenthesisToken => {
                let open_parenthesis_node = Box::new(ParenthesisNode::new(self.current_token()));
                let expression = self.parse_expression(0)?;

                let token = self.current_token();
                if token.kind != Kind::CloseParenthesisToken {
                    Err(format!("Close parenthesis expected"))
                } else {
                    let close_parenthesis_node = Box::new(ParenthesisNode::new(token));

                    Ok(Box::new(ParenthesizedExpressionNode::new(
                        open_parenthesis_node,
                        expression,
                        close_parenthesis_node,
                    )))
                }
            }
            _ => Err(format!("Operator or expression expected")),
        }
    }
}

fn get_binary_operator_precedence(kind: Kind) -> u8 {
    match kind {
        Kind::SlashToken | Kind::StarToken | Kind::ModToken => 2,
        Kind::MinusToken | Kind::PlusToken => 1,
        _ => 0,
    }
}
