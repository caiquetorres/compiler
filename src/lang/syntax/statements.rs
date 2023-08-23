use super::{
    expressions::{Expression, TreeDisplay},
    lexer::Token,
};

pub struct LetKeyword(pub Token);

pub struct Colon(pub Token);

pub struct Identifier(pub Token);

pub struct AssignmentOperator(pub Token);

pub struct Semicolon(pub Token);

pub struct Brace(pub Token);

pub enum Let {
    TypedWithValue(
        LetKeyword,
        Identifier,
        Colon,
        Identifier,
        AssignmentOperator,
        Expression,
        Semicolon,
    ),
    TypedWithoutValue(LetKeyword, Identifier, Colon, Identifier, Semicolon),
    UntypedWithValue(
        LetKeyword,
        Identifier,
        AssignmentOperator,
        Expression,
        Semicolon,
    ),
}

pub enum Statement {
    Let(Let),
    Block(Brace, Vec<Statement>, Brace),
    Assignment(Identifier, AssignmentOperator, Expression, Semicolon),
}

impl TreeDisplay for Statement {
    fn display(&self, layer: usize) {
        match self {
            Self::Let(l) => match l {
                Let::UntypedWithValue(_, identifier, operator, expression, _) => {
                    let id = identifier.0.value.as_ref().unwrap();
                    println!("{}LetStatement ({})", "  ".repeat(layer), id);

                    let op = operator.0.value.as_ref().unwrap();
                    println!("{}AssignmentOperator ({})", "  ".repeat(layer + 1), op);

                    expression.display(layer + 1);

                    println!("{}Semicolon (;)", "  ".repeat(layer));
                }
                Let::TypedWithValue(_, identifier, _, type_identifier, operator, expression, _) => {
                    let id = identifier.0.value.as_ref().unwrap();
                    let t = type_identifier.0.value.as_ref().unwrap();
                    println!("{}LetStatement ({}) ({})", "  ".repeat(layer), id, t);

                    let op = operator.0.value.as_ref().unwrap();
                    println!("{}AssignmentOperator ({})", "  ".repeat(layer + 1), op);

                    expression.display(layer + 1);
                    println!("{}Semicolon (;)", "  ".repeat(layer));
                }
                Let::TypedWithoutValue(_, identifier, _, type_identifier, _) => {
                    let id = identifier.0.value.as_ref().unwrap();
                    let t = type_identifier.0.value.as_ref().unwrap();
                    println!("{}LetStatement ({}) ({})", "  ".repeat(layer), id, t);
                    println!("{}Semicolon (;)", "  ".repeat(layer));
                }
            },
            Self::Block(_, statements, _) => {
                println!("{}BlockStatement", "  ".repeat(layer));
                for statement in statements {
                    statement.display(layer + 1)
                }
            }
            Self::Assignment(identifier, operator, expression, _) => {
                let id = identifier.0.value.as_ref().unwrap();
                println!("{}AssignmentStatement ({})", "  ".repeat(layer), id);

                let op = operator.0.value.as_ref().unwrap();
                println!("{}AssignmentOperator ({})", "  ".repeat(layer + 1), op);
                expression.display(layer + 1);

                println!("{}Semicolon (;)", "  ".repeat(layer));
            }
        }
    }
}
