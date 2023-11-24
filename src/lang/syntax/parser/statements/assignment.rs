use crate::lang::syntax::parser::expressions::expression::Expression;
use crate::lang::syntax::parser::shared::assignment_operator::AssignmentOperator;
use crate::lang::syntax::parser::shared::identifier::Identifier;
use crate::lang::syntax::tree_display::TreeDisplay;

#[derive(Clone, Debug)]
pub struct Assignment {
    pub identifier: Identifier,
    pub operator: AssignmentOperator,
    pub expression: Expression,
}

impl Assignment {
    pub fn new(
        identifier: Identifier,
        operator: AssignmentOperator,
        expression: Expression,
    ) -> Self {
        Self {
            identifier,
            operator,
            expression,
        }
    }
}

impl TreeDisplay for Assignment {
    fn display(&self, layer: usize) {
        let id = self.identifier.name.clone();
        println!("{}AssignmentStatement ({})", " ".repeat(layer), id);

        self.operator.display(layer + 2);
        self.expression.display(layer + 2);
    }
}
