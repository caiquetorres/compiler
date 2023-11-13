use crate::lang::syntax::parser::expressions::expression::Expression;
use crate::lang::syntax::parser::shared::assignment_operator::AssignmentOperator;
use crate::lang::syntax::parser::shared::identifier::Identifier;
use crate::lang::syntax::tree_display::TreeDisplay;

pub enum Const {
    WithValue(
        Identifier,
        Option<Identifier>,
        AssignmentOperator,
        Expression,
    ),
    WithoutValue(Identifier, Identifier),
}

impl TreeDisplay for Const {
    fn display(&self, layer: usize) {
        match &self {
            Const::WithValue(identifier, opt_type, operator, expression) => {
                let id = identifier.0.value.as_ref().unwrap();

                match opt_type {
                    None => {
                        println!("{}ConstStatement ({})", " ".repeat(layer), id);
                    }
                    Some(t) => {
                        println!(
                            "{}ConstStatement ({}) ({})",
                            " ".repeat(layer),
                            id,
                            t.0.value.as_ref().unwrap()
                        );
                    }
                }

                operator.display(layer + 2);
                expression.display(layer + 2);
            }
            Const::WithoutValue(identifier, type_identifier) => {
                let id = identifier.0.value.as_ref().unwrap();
                let type_id = type_identifier.0.value.as_ref().unwrap();
                println!("{}ConstStatement ({}) ({})", " ".repeat(layer), id, type_id);
            }
        }
    }
}
