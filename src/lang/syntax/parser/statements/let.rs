use crate::lang::syntax::parser::expressions::expression::Expression;
use crate::lang::syntax::parser::shared::identifier::Identifier;

use crate::lang::syntax::parser::shared::r#type::Type;
use crate::lang::syntax::tree_display::TreeDisplay;

#[derive(Clone, Debug)]
pub struct Let {
    pub identifier: Identifier,
    pub r#type: Option<Type>,
    pub expression: Option<Expression>,
}

impl Let {
    pub fn new(
        identifier: Identifier,
        r#type: Option<Type>,
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

        if let Some(r#type) = &self.r#type {
            r#type.display(0);
        }

        if let Some(expression) = &self.expression {
            expression.display(layer + 1);
        }
    }
}
