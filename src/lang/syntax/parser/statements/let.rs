use crate::lang::syntax::parser::expressions::expression::Expression;
use crate::lang::syntax::parser::shared::assignment_operator::AssignmentOperator;
use crate::lang::syntax::parser::shared::identifier::Identifier;
use crate::lang::syntax::tree_display::TreeDisplay;

pub enum Let {
    WithValue(
        Identifier,
        Option<Identifier>,
        AssignmentOperator,
        Expression,
    ),
    WithoutValue(Identifier, Identifier),
}

impl TreeDisplay for Let {
    fn display(&self, layer: usize) {
        match &self {
            Let::WithValue(identifier, opt_type, operator, expression) => {
                let id = identifier.token.value.clone();

                match opt_type {
                    None => {
                        println!("{}LetStatement ({})", " ".repeat(layer), id);
                    }
                    Some(t) => {
                        println!(
                            "{}LetStatement ({}) ({})",
                            " ".repeat(layer),
                            id,
                            t.token.value
                        );
                    }
                }

                operator.display(layer + 2);
                expression.display(layer + 2);
            }
            Let::WithoutValue(identifier, type_identifier) => {
                let id = identifier.token.value.clone();
                let type_id = type_identifier.token.value.clone();
                println!("{}LetStatement ({}) ({})", " ".repeat(layer), id, type_id);
            }
        }
    }
}
