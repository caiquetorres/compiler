use crate::lang::{
    assignment_operator_node::AssignmentOperatorNode, binary_expression_node::BinaryExpressionNode,
    brace_node::BraceNode, let_node::LetNode, operator_node::OperatorNode,
    semicolon_node::SemicolonNode,
};

use super::{
    block_statement_node::BlockStatementNode, identifier_node::IdentifierNode, kind::Kind,
    lexer::Lexer, literal_expression_node::LiteralExpressionNode, node::Node,
    parenthesis_node::ParenthesisNode, parenthesized_expression_node::ParenthesizedExpressionNode,
    token::Token, tree::Tree, unary_expression_node::UnaryExpressionNode,
    variable_declaration_statement_node::VariableDeclarationStatementNode,
};

fn get_unary_operator_precedence(kind: Kind) -> u32 {
    match kind {
        Kind::PlusToken | Kind::MinusToken | Kind::LogicalNotToken | Kind::BitwiseNotToken => 10,
        _ => 0,
    }
}

fn get_binary_operator_precedence(kind: Kind) -> u32 {
    match kind {
        Kind::SlashToken | Kind::StarToken | Kind::ModToken => 9,
        Kind::MinusToken | Kind::PlusToken => 8,
        Kind::LogicalGreaterThan
        | Kind::LogicalGreaterThanOrEquals
        | Kind::LogicalLessThan
        | Kind::LogicalLessThanOrEquals => 7,
        Kind::LogicalEquals | Kind::LogicalNotEquals => 6,
        Kind::BitwiseAndToken => 5,
        Kind::BitwiseXorToken => 4,
        Kind::BitwiseOrToken => 3,
        Kind::LogicalAndToken => 2,
        Kind::LogicalOrToken => 1,
        _ => 0,
    }
}

pub struct Parser {
    current_position: usize,
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
            current_position: 0,
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

        let expression = self.parse_block()?;
        let tree = Tree::new(expression);
        return Ok(tree);
    }

    fn current_token(&mut self) -> Token {
        match self.tokens.get(self.current_position as usize) {
            Some(token) => token.clone(),
            None => Token::new(Kind::EndOfFileToken, ""),
        }
    }

    fn next_token(&mut self) -> Token {
        let token = self.current_token();
        self.current_position += 1;
        token
    }

    fn parse_block(&mut self) -> Result<Box<dyn Node>, String> {
        let token = self.current_token();

        if token.kind != Kind::OpenBracesToken {
            return Err("Block expected".to_string());
        }

        let open_brace_token = self.next_token();
        let open_brace_node = Box::new(BraceNode::new(open_brace_token));

        let mut statements: Vec<Box<dyn Node>> = vec![];

        while self.current_token().kind != Kind::CloseBracesToken {
            let statement = self.parse_statement()?;
            statements.push(statement);
        }

        let close_brace_token = self.next_token();
        let close_brace_node = Box::new(BraceNode::new(close_brace_token));

        Ok(Box::new(BlockStatementNode::new(
            open_brace_node,
            statements,
            close_brace_node,
        )))
    }

    fn parse_statement(&mut self) -> Result<Box<dyn Node>, String> {
        match self.current_token().kind {
            Kind::OpenBracesToken => self.parse_block(),
            Kind::LetToken => {
                let mut token = self.next_token();
                let let_node = Box::new(LetNode::new(token));

                token = self.next_token();

                if token.kind != Kind::IdentifierToken {
                    return Err("Expected identifier".to_string());
                }

                let id_node = Box::new(IdentifierNode::new(token));

                token = self.next_token();

                if token.kind != Kind::EqualsToken {
                    return Err("Assignment operator expected".to_string());
                }

                let assignment_node = Box::new(AssignmentOperatorNode::new(token));

                let expression_node = self.parse_expression(0)?;

                token = self.next_token();

                if token.kind != Kind::SemicolonToken {
                    return Err("Semicolon expected".to_string());
                }

                let semicolon_node = Box::new(SemicolonNode::new(token));

                Ok(Box::new(VariableDeclarationStatementNode::new(
                    let_node,
                    id_node,
                    assignment_node,
                    expression_node,
                    semicolon_node,
                )))
            }
            _ => Err("Expected statement".to_string()),
        }
    }

    fn parse_expression(&mut self, parent_precedence: u32) -> Result<Box<dyn Node>, String> {
        let mut left: Box<dyn Node>;
        let mut token = self.current_token();

        // Checks whether the current token is unary or not.
        let unary_precedence = get_unary_operator_precedence(token.kind);
        if unary_precedence != 0 && unary_precedence >= parent_precedence {
            token = self.next_token();
            left = Box::new(UnaryExpressionNode::new(
                Box::new(OperatorNode::new(token)),
                self.parse_expression(unary_precedence)?,
            ));
        } else {
            left = self.parse_factor()?;
        }

        token = self.current_token();
        let mut precedence = get_binary_operator_precedence(token.kind);

        while precedence != 0 && precedence > parent_precedence {
            let operator_token = self.next_token();
            let operator = Box::new(OperatorNode::new(operator_token));
            let right = self.parse_expression(precedence)?;

            left = Box::new(BinaryExpressionNode::new(left, operator, right));

            token = self.current_token();
            precedence = get_binary_operator_precedence(token.kind);
        }

        Ok(left)
    }

    fn parse_factor(&mut self) -> Result<Box<dyn Node>, String> {
        let mut token = self.next_token();

        match token.kind {
            Kind::IdentifierToken => Ok(Box::new(IdentifierNode::new(token))),
            Kind::NumberToken | Kind::TrueToken | Kind::FalseToken => {
                Ok(Box::new(LiteralExpressionNode::new(token)))
            }
            Kind::OpenParenthesisToken => {
                let open_parenthesis_node = Box::new(ParenthesisNode::new(token));
                let expression = self.parse_expression(0)?;

                token = self.next_token();

                if token.kind == Kind::CloseParenthesisToken {
                    let close_parenthesis_node = Box::new(ParenthesisNode::new(token));
                    return Ok(Box::new(ParenthesizedExpressionNode::new(
                        open_parenthesis_node,
                        expression,
                        close_parenthesis_node,
                    )));
                }

                Err(format!("Close parenthesis expected"))
            }
            _ => Err(format!("Expression expected")),
        }
    }
}
