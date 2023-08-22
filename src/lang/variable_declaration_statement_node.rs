use super::{
    assignment_operator_node::AssignmentOperatorNode, colon_node::ColonNode,
    identifier_node::IdentifierNode, kind::Kind, let_node::LetNode, node::Node,
    semicolon_node::SemicolonNode, token::Token,
};

pub struct VariableDeclarationStatementNode {
    token: Token,
    pub let_keyword: Box<LetNode>,
    pub identifier: Box<IdentifierNode>,
    pub colon_token: Option<Box<ColonNode>>,
    pub type_token: Option<Box<IdentifierNode>>,
    pub equals: Option<Box<AssignmentOperatorNode>>,
    pub expression: Option<Box<dyn Node>>,
    pub semicolon: Box<SemicolonNode>,
}

impl VariableDeclarationStatementNode {
    pub fn new(
        let_keyword: Box<LetNode>,
        identifier: Box<IdentifierNode>,
        colon: Option<Box<ColonNode>>,
        type_token: Option<Box<IdentifierNode>>,
        equals: Option<Box<AssignmentOperatorNode>>,
        expression: Option<Box<dyn Node>>,
        semicolon: Box<SemicolonNode>,
    ) -> Self {
        Self {
            let_keyword,
            identifier,
            colon_token: colon,
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
        let mut v: Vec<&dyn Node> = vec![self.let_keyword.as_ref(), self.identifier.as_ref()];

        if self.colon_token.is_some() {
            v.push(self.colon_token.as_ref().unwrap().as_ref());
        }

        if self.type_token.is_some() {
            v.push(self.type_token.as_ref().unwrap().as_ref());
        }

        if self.equals.is_some() {
            v.push(self.equals.as_ref().unwrap().as_ref());
        }

        if self.expression.is_some() {
            v.push(self.expression.as_ref().unwrap().as_ref());
        }

        v.push(self.semicolon.as_ref());

        v
    }
}
