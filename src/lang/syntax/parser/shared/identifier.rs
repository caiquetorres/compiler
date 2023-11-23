use crate::lang::syntax::lexer::token::Token;
use crate::lang::syntax::parser::expressions::expression::Expression;
use crate::lang::syntax::tree_display::TreeDisplay;

#[derive(Clone)]
pub enum IdentifierMeta {
    Index(Box<Expression>, Box<Option<IdentifierMeta>>),
}

#[derive(Clone)]
pub struct Identifier {
    pub token: Token,
    pub name: String,
    pub meta: Option<IdentifierMeta>,
}

impl Identifier {
    pub fn new(token: Token, meta: Option<IdentifierMeta>) -> Self {
        let name = token.value.clone();
        Self { token, name, meta }
    }
}

impl TreeDisplay for Identifier {
    fn display(&self, layer: usize) {
        let value = self.name.clone();
        println!("{}Identifier ({})", " ".repeat(layer), value);
    }
}
