use super::{kind::Kind, node::Node, token::Token};

pub struct VariableDeclarationStatementNode {
    token: Token,
    pub let_keyword: Token,
    pub identifier: Token,
    pub colon_token: Option<Token>,
    pub type_token: Option<Token>,
    pub equals: Option<Token>,
    pub expression: Option<Box<dyn Node>>,
    pub semicolon: Token,
}

impl VariableDeclarationStatementNode {
    pub fn new(
        let_keyword: Token,
        identifier: Token,
        colon_token: Option<Token>,
        type_token: Option<Token>,
        equals: Option<Token>,
        expression: Option<Box<dyn Node>>,
        semicolon: Token,
    ) -> Self {
        Self {
            let_keyword,
            identifier,
            colon_token,
            type_token,
            equals,
            expression,
            semicolon,
            token: Token::new(Kind::VariableAssignment, ""),
        }
    }
}

impl ToString for VariableDeclarationStatementNode {
    fn to_string(&self) -> String {
        "VariableDeclarationStatementNode".to_string()
    }
}

impl Node for VariableDeclarationStatementNode {
    fn get_token(&self) -> &Token {
        &self.token
    }

    fn get_children(&self) -> Vec<&dyn Node> {
        if self.expression.is_none() {
            vec![]
        } else {
            vec![self.expression.as_ref().unwrap().as_ref()]
        }
    }
}
