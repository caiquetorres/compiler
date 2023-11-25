use crate::lang::syntax::expressions::expression::Expression;
use crate::lang::syntax::shared::identifier::Identifier;

use crate::lang::syntax::shared::syntax_type::SyntaxType;
use crate::lang::syntax::tree_display::TreeDisplay;

#[derive(Clone, Debug)]
pub struct Let {
    pub identifier: Identifier,
    pub r#type: Option<SyntaxType>,
    pub expression: Option<Expression>,
}

impl Let {
    pub fn new(
        identifier: Identifier,
        r#type: Option<SyntaxType>,
        expression: Option<Expression>,
    ) -> Self {
        Self {
            identifier,
            r#type,
            expression,
        }
    }
}

impl TreeDisplay for Let {
    fn display(&self, layer: usize) {
        let identifier_name = self.identifier.name.clone();

        if let Some(r#type) = &self.r#type {
            println!(
                "{}LetStatement ({}: {})",
                "  ".repeat(layer),
                identifier_name,
                r#type.to_string()
            );
        } else {
            println!("{}LetStatement ({})", "  ".repeat(layer), identifier_name);
        }

        if let Some(expression) = &self.expression {
            expression.display(layer + 1);
        }
    }
}
