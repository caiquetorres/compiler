use crate::lang::syntax::parser::expressions::expression::Expression;
use crate::lang::syntax::parser::shared::assignment_operator::AssignmentOperator;
use crate::lang::syntax::parser::shared::identifier::Identifier;
use crate::lang::syntax::tree_display::TreeDisplay;

pub struct Assignment(pub Identifier, pub AssignmentOperator, pub Expression);

impl TreeDisplay for Assignment {
    fn display(&self, layer: usize) {
        let id = self.0 .0.value.as_ref().unwrap();
        println!("{}AssignmentStatement ({})", " ".repeat(layer), id);

        self.1.display(layer + 2);
        self.2.display(layer + 2);
    }
}
