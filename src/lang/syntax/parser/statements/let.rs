use crate::lang::syntax::parser::expressions::expression::Expression;
use crate::lang::syntax::parser::shared::identifier::Identifier;

use crate::lang::syntax::tree_display::TreeDisplay;

#[derive(Clone)]
pub struct Let {
    pub identifier: Identifier,
    pub type_identifier: Option<Identifier>,
    pub expression: Option<Expression>,
}

impl Let {
    pub fn new(
        identifier: Identifier,
        type_identifier: Option<Identifier>,
        expression: Option<Expression>,
    ) -> Self {
        Self {
            identifier,
            type_identifier,
            expression,
        }
    }
}

impl TreeDisplay for Let {
    fn display(&self, layer: usize) {
        let identifier_name = self.identifier.name.clone();

        print!("{}LetStatement ({})", " ".repeat(layer), identifier_name);

        if let Some(type_identifier) = &self.type_identifier {
            let type_identifier_name = type_identifier.name.clone();
            print!(" ({})", type_identifier_name);
        }

        if let Some(expression) = &self.expression {
            println!("");
            expression.display(layer + 2);
        }

        println!("");
    }
}
