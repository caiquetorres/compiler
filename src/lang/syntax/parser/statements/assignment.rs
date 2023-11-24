use crate::lang::syntax::parser::expressions::expression::Expression;
use crate::lang::syntax::parser::shared::assignment_operator::AssignmentOperator;
use crate::lang::syntax::tree_display::TreeDisplay;

#[derive(Clone, Debug)]
pub struct Assignment {
    pub left: Expression,
    pub operator: AssignmentOperator,
    pub right: Expression,
}

impl Assignment {
    pub fn new(left: Expression, operator: AssignmentOperator, right: Expression) -> Self {
        Self {
            left,
            operator,
            right,
        }
    }
}

impl TreeDisplay for Assignment {
    fn display(&self, layer: usize) {
        // let id = self.left.name.clone();
        // println!("{}AssignmentStatement ({})", " ".repeat(layer), id);

        // self.operator.display(layer + 2);
        // self.expression.display(layer + 2);
    }
}
