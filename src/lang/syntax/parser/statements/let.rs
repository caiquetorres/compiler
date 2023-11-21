use crate::lang::syntax::parser::expressions::expression::Expression;
use crate::lang::syntax::parser::shared::identifier::Identifier;

use crate::lang::syntax::tree_display::TreeDisplay;

#[derive(Clone)]
pub enum Let {
    WithValue(Identifier, Option<Identifier>, Expression),
    WithoutValue(Identifier, Identifier),
}

impl TreeDisplay for Let {
    fn display(&self, layer: usize) {
        match &self {
            Let::WithValue(identifier, opt_type, expression) => {
                let id = identifier.name.clone();

                match opt_type {
                    None => {
                        println!("{}LetStatement ({})", " ".repeat(layer), id);
                    }
                    Some(t) => {
                        println!("{}LetStatement ({}) ({})", " ".repeat(layer), id, t.name);
                    }
                }

                expression.display(layer + 2);
            }
            Let::WithoutValue(identifier, type_identifier) => {
                let id = identifier.name.clone();
                let type_id = type_identifier.name.clone();
                println!("{}LetStatement ({}) ({})", " ".repeat(layer), id, type_id);
            }
        }
    }
}
