use super::{
    assignment_operator_node::AssignmentOperatorNode, identifier_node::IdentifierNode, kind::Kind,
    let_node::LetNode, node::Node, semicolon_node::SemicolonNode, token::Token,
};

pub struct VariableDeclarationStatementNode {
    token: Token,
    pub let_keyword: Box<LetNode>,
    // REVIEW: Should we add types?
    pub identifier: Box<IdentifierNode>,
    pub equals: Box<AssignmentOperatorNode>,
    pub expression: Box<dyn Node>,
    pub semicolon: Box<SemicolonNode>,
}

impl VariableDeclarationStatementNode {
    pub fn new(
        let_keyword: Box<LetNode>,
        identifier: Box<IdentifierNode>,
        equals: Box<AssignmentOperatorNode>,
        expression: Box<dyn Node>,
        semicolon: Box<SemicolonNode>,
    ) -> Self {
        Self {
            let_keyword,
            identifier,
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
        vec![
            self.let_keyword.as_ref(),
            self.identifier.as_ref(),
            self.equals.as_ref(),
            self.expression.as_ref(),
            self.semicolon.as_ref(),
        ]
    }
}
