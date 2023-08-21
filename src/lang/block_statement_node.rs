use super::{brace_node::BraceNode, kind::Kind, node::Node, token::Token};

pub struct BlockStatementNode {
    token: Token,
    pub open_brace: Box<BraceNode>,
    pub statements: Vec<Box<dyn Node>>, // REVIEW: Pointer to pointer?
    pub close_brace: Box<BraceNode>,
}

impl BlockStatementNode {
    pub fn new(
        open_brace: Box<BraceNode>,
        statements: Vec<Box<dyn Node>>,
        close_brace: Box<BraceNode>,
    ) -> Self {
        Self {
            open_brace,
            statements,
            close_brace,
            token: Token::new(Kind::BlockStatement, ""),
        }
    }
}

impl ToString for BlockStatementNode {
    fn to_string(&self) -> String {
        "BlockStatementNode".to_string()
    }
}

impl Node for BlockStatementNode {
    fn get_token(&self) -> &Token {
        &self.token
    }

    fn get_children(&self) -> Vec<&dyn Node> {
        self.statements
            .iter()
            .map(|statement| statement.as_ref())
            .collect()
    }
}
