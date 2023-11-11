use super::lexer::Token;
use super::statements::{Identifier, Params};
use super::tree_display::TreeDisplay;

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
            Self::Literal(token) => display_literal_expression(layer, token),
            Self::Identifier(token) => display_identifier_expression(layer, token),
            Self::Unary(op, ex) => display_unary_expression(layer, op, ex),
            Self::Binary(left, operator, right) => {
                display_binary_expression(layer, left, operator, right)
            }
            Self::Parenthesized(expression) => display_parenthesized_expression(layer, expression),
            Self::FunctionCall(identifier, params) => {
                display_function_call_expression(layer, identifier, params)
            }
        }
    }
}

fn display_literal_expression(layer: usize, token: &Token) {
    let value = token.value.as_ref().unwrap();
    println!("{}Literal ({})", "  ".repeat(layer), value);
}

fn display_identifier_expression(layer: usize, token: &Token) {
    let value = token.value.as_ref().unwrap();
    println!("{}Identifier ({})", "  ".repeat(layer), value);
}

fn display_unary_expression(layer: usize, operator: &UnaryOperator, expression: &Expression) {
    println!("{}UnaryExpression", "  ".repeat(layer));
    let value = operator.0.value.as_ref().unwrap();
    println!("{}UnaryOperator ({})", "  ".repeat(layer + 1), value);
    expression.display(layer + 1);
}

fn display_binary_expression(
    layer: usize,
    left: &Expression,
    operator: &BinaryOperator,
    right: &Expression,
) {
    println!("{}BinaryExpression", "  ".repeat(layer));
    left.display(layer + 1);
    let value = operator.0.value.as_ref().unwrap();
    println!("{}BinaryOperator ({})", "  ".repeat(layer + 1), value);
    right.display(layer + 1);
}

fn display_parenthesized_expression(layer: usize, expression: &Expression) {
    println!("{}Parenthesized", "  ".repeat(layer));
    expression.display(layer + 1);
}

fn display_function_call_expression(layer: usize, identifier: &Identifier, params: &Params) {
    let id = identifier.0.value.as_ref().unwrap();
    let expressions = &params.0;
    println!("{}FunctionCallExpression ({})", "  ".repeat(layer), id);

    for expression in expressions {
        expression.display(layer + 1);
    }
}
