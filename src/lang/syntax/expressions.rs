use super::lexer::Token;

pub struct UnaryOperator(pub Token);

pub struct BinaryOperator(pub Token);

pub struct Parenthesis(pub Token);

pub enum Expression {
    Literal(Token),
    Identifier(Token),
    Unary(UnaryOperator, Box<Expression>),
    Binary(Box<Expression>, BinaryOperator, Box<Expression>),
    Parenthesized(Parenthesis, Box<Expression>, Parenthesis),
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
            Self::Parenthesized(_, expression, _) => {
                println!("{}Parenthesized", "  ".repeat(layer));
                expression.display(layer + 1);
            }
        }
    }
}

pub trait TreeDisplay {
    fn display(&self, layer: usize);
}
