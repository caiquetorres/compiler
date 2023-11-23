use crate::lang::syntax::parser::expressions::expression::Expression;
use crate::lang::syntax::parser::shared::assignment_operator::AssignmentOperator;
use crate::lang::syntax::parser::shared::identifier::Identifier;
use crate::lang::syntax::parser::shared::r#type::Type;
use crate::lang::syntax::tree_display::TreeDisplay;

#[derive(Clone)]
pub struct Const {
    pub identifier: Identifier,
    pub r#type: Option<Type>,
    pub operator: AssignmentOperator,
    pub expression: Expression,
}

impl Const {
    pub fn new(
        identifier: Identifier,
        r#type: Option<Type>,
        operator: AssignmentOperator,
        expression: Expression,
    ) -> Self {
        Self {
            identifier,
            r#type,
            operator,
            expression,
        }
    }
}

impl TreeDisplay for Const {
    fn display(&self, layer: usize) {
        // TODO: Reimplement the display of the constant

        // let id = self.identifier.name.clone();

        // match &self.r#type {
        //     None => println!("{}ConstStatement ({})", " ".repeat(layer), id),
        //     Some(t) => println!("{}ConstStatement ({}) ({})", " ".repeat(layer), id, t.name),
        // }

        self.operator.display(layer + 2);
        self.expression.display(layer + 2);
    }
}
