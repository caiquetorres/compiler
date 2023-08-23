use super::{
    lexer::Token,
    statements::{Identifier, Params},
};

pub struct UnaryOperator(pub Token);

pub struct BinaryOperator(pub Token);

pub enum Expression {
    Literal(Token),
    Identifier(Token),
    Unary(UnaryOperator, Box<Expression>),
    Binary(Box<Expression>, BinaryOperator, Box<Expression>),
    Parenthesized(Box<Expression>),
    FunctionCall(Identifier, Params),
}

impl TreeDisplay for Expression {
    fn display(&self, layer: usize) {
        match self {
            Self::Literal(token) => {
                let value = token.value.as_ref().unwrap();
                println!("{}Literal ({})", "  ".repeat(layer), value);
            }
            Self::Identifier(token) => {
                let value = token.value.as_ref().unwrap();
                println!("{}Identifier ({})", "  ".repeat(layer), value);
            }
            Self::Unary(operator, expression) => {
                println!("{}Unary", "  ".repeat(layer));

                let value = operator.0.value.as_ref().unwrap();
                println!("{}UnaryOperator ({})", "  ".repeat(layer + 1), value);

                expression.display(layer + 1);
            }
            Self::Binary(left, operator, right) => {
                println!("{}Binary", "  ".repeat(layer));

                left.display(layer + 1);

                let value = operator.0.value.as_ref().unwrap();
                println!("{}BinaryOperator ({})", "  ".repeat(layer + 1), value);

                right.display(layer + 1);
            }
            Self::Parenthesized(expression) => {
                println!("{}Parenthesized", "  ".repeat(layer));
                expression.display(layer + 1);
            }
            Self::FunctionCall(identifier, params) => {
                let id = identifier.0.value.as_ref().unwrap();
                let expressions = &params.0;
                println!("{}FunctionCallExpression ({})", "  ".repeat(layer), id);

                for expression in expressions {
                    expression.display(layer + 1);
                }
            }
        }
    }
}

pub trait TreeDisplay {
    fn display(&self, layer: usize);
}
