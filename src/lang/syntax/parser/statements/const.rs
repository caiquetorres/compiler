use crate::lang::syntax::parser::expressions::expression::Expression;
use crate::lang::syntax::parser::shared::assignment_operator::AssignmentOperator;
use crate::lang::syntax::parser::shared::identifier::Identifier;
use crate::lang::syntax::tree_display::TreeDisplay;

pub struct Const {
    pub identifier: Identifier,
    pub type_identifier: Option<Identifier>,
    pub operator: AssignmentOperator,
    pub expression: Expression,
}

impl Const {
    pub fn new(
        identifier: Identifier,
        type_identifier: Option<Identifier>,
        operator: AssignmentOperator,
        expression: Expression,
    ) -> Self {
        Self {
            identifier,
            type_identifier,
            operator,
            expression,
        }
    }
}

impl TreeDisplay for Const {
    fn display(&self, layer: usize) {
        let id = self.identifier.name.clone();

        match &self.type_identifier {
            None => println!("{}ConstStatement ({})", " ".repeat(layer), id),
            Some(t) => println!("{}ConstStatement ({}) ({})", " ".repeat(layer), id, t.name),
        }

        self.operator.display(layer + 2);
        self.expression.display(layer + 2);
    }
}
