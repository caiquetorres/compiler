use super::expressions::{Expression, Parenthesis, TreeDisplay};
use super::lexer::Token;

pub struct LetKeyword(pub Token);

pub struct ReturnKeyword(pub Token);

pub struct Colon(pub Token);

pub struct Identifier(pub Token);

pub struct AssignmentOperator(pub Token);

pub struct Semicolon(pub Token);

pub struct Brace(pub Token);

pub struct FunKeyword(pub Token);

pub struct Block(pub Brace, pub Vec<Statement>, pub Brace);

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

pub enum Return {
    WithExpression(ReturnKeyword, Expression, Semicolon),
    WithoutExpression(ReturnKeyword, Semicolon),
}

pub enum Function {
    Typed(
        FunKeyword,
        Identifier,
        Parenthesis,
        Parenthesis,
        Colon,
        Identifier,
        Block,
    ),
    Untyped(FunKeyword, Identifier, Parenthesis, Parenthesis, Block),
}

// TODO: Functions need arguments
pub enum TopLevelStatement {
    Function(Function),
}

impl TreeDisplay for TopLevelStatement {
    fn display(&self, layer: usize) {
        match self {
            Self::Function(function) => match function {
                Function::Typed(_, identifier, _, _, _, type_id, block) => {
                    let id = identifier.0.value.as_ref().unwrap();
                    let t = type_id.0.value.as_ref().unwrap();
                    println!("{}FunctionDeclaration ({}) ({})", "  ".repeat(layer), id, t);

                    let statements = &block.1;
                    for statement in statements {
                        statement.display(layer + 1);
                    }
                }
                Function::Untyped(_, identifier, _, _, block) => {
                    let id = identifier.0.value.as_ref().unwrap();
                    let statements = &block.1;
                    println!("{}FunctionDeclaration ({})", "  ".repeat(layer), id);

                    for statement in statements {
                        statement.display(layer + 1);
                    }
                }
            },
        }
    }
}

pub enum Statement {
    Let(Let),
    Block(Block),
    Assignment(Identifier, AssignmentOperator, Expression, Semicolon),
    Return(Return),
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
                }
                Let::TypedWithValue(_, identifier, _, type_identifier, operator, expression, _) => {
                    let id = identifier.0.value.as_ref().unwrap();
                    let t = type_identifier.0.value.as_ref().unwrap();
                    println!("{}LetStatement ({}) ({})", "  ".repeat(layer), id, t);

                    let op = operator.0.value.as_ref().unwrap();
                    println!("{}AssignmentOperator ({})", "  ".repeat(layer + 1), op);

                    expression.display(layer + 1);
                }
                Let::TypedWithoutValue(_, identifier, _, type_identifier, _) => {
                    let id = identifier.0.value.as_ref().unwrap();
                    let t = type_identifier.0.value.as_ref().unwrap();
                    println!("{}LetStatement ({}) ({})", "  ".repeat(layer), id, t);
                }
            },
            Self::Block(b) => {
                println!("{}BlockStatement", "  ".repeat(layer));
                for statement in &b.1 {
                    statement.display(layer + 1)
                }
            }
            Self::Assignment(identifier, operator, expression, _) => {
                let id = identifier.0.value.as_ref().unwrap();
                println!("{}AssignmentStatement ({})", "  ".repeat(layer), id);

                let op = operator.0.value.as_ref().unwrap();
                println!("{}AssignmentOperator ({})", "  ".repeat(layer + 1), op);
                expression.display(layer + 1);
            }
            Self::Return(r) => match r {
                Return::WithExpression(_, expression, _) => {
                    println!("{}ReturnStatement", "  ".repeat(layer));
                    expression.display(layer + 1);
                }
                Return::WithoutExpression(_, _) => {
                    println!("{}ReturnStatement", "  ".repeat(layer))
                }
            },
        }
    }
}
