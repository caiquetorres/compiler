use super::expressions::{Expression, TreeDisplay};
use super::lexer::Token;

pub struct Identifier(pub Token);

pub struct AssignmentOperator(pub Token);

pub struct Block(pub Vec<Statement>);

pub enum Let {
    TypedWithValue(Identifier, Identifier, AssignmentOperator, Expression),
    TypedWithoutValue(Identifier, Identifier),
    UntypedWithValue(Identifier, AssignmentOperator, Expression),
}

pub enum Return {
    WithExpression(Expression),
    WithoutExpression,
}

pub struct ParamDeclaration(pub Identifier, pub Identifier);

pub struct ParamsDeclaration(pub Vec<ParamDeclaration>);

pub struct Params(pub Vec<Expression>);

pub enum Function {
    Typed(Identifier, ParamsDeclaration, Identifier, Block),
    Untyped(Identifier, ParamsDeclaration, Block),
}

// TODO: Functions need arguments
pub enum TopLevelStatement {
    Function(Function),
}

impl TreeDisplay for TopLevelStatement {
    fn display(&self, layer: usize) {
        match self {
            Self::Function(function) => match function {
                Function::Typed(identifier, params, type_id, block) => {
                    let id = identifier.0.value.as_ref().unwrap();
                    let t = type_id.0.value.as_ref().unwrap();
                    println!("{}FunctionDeclaration ({}) ({})", "  ".repeat(layer), id, t);

                    for param in &params.0 {
                        let name = &param.0 .0.value.as_ref().unwrap();
                        let t = &param.1 .0.value.as_ref().unwrap();
                        println!(
                            "{}ParamDeclaration ({}) ({})",
                            "  ".repeat(layer + 1),
                            name,
                            t
                        );
                    }

                    let statements = &block.0;
                    for statement in statements {
                        statement.display(layer + 1);
                    }
                }
                Function::Untyped(identifier, params, block) => {
                    let id = identifier.0.value.as_ref().unwrap();
                    let statements = &block.0;
                    println!("{}FunctionDeclaration ({})", "  ".repeat(layer), id);

                    for param in &params.0 {
                        let name = &param.0 .0.value.as_ref().unwrap();
                        let t = &param.1 .0.value.as_ref().unwrap();
                        println!(
                            "{}ParamDeclaration ({}) ({})",
                            "  ".repeat(layer + 1),
                            name,
                            t
                        );
                    }

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
    Assignment(Identifier, AssignmentOperator, Expression),
    Return(Return),
    If(Expression, Block),
    FunctionCall(Identifier, Params),
}

impl TreeDisplay for Statement {
    fn display(&self, layer: usize) {
        match self {
            Self::Let(l) => match l {
                Let::UntypedWithValue(identifier, operator, expression) => {
                    let id = identifier.0.value.as_ref().unwrap();
                    println!("{}LetStatement ({})", "  ".repeat(layer), id);

                    let op = operator.0.value.as_ref().unwrap();
                    println!("{}AssignmentOperator ({})", "  ".repeat(layer + 1), op);

                    expression.display(layer + 1);
                }
                Let::TypedWithValue(identifier, type_identifier, operator, expression) => {
                    let id = identifier.0.value.as_ref().unwrap();
                    let t = type_identifier.0.value.as_ref().unwrap();
                    println!("{}LetStatement ({}) ({})", "  ".repeat(layer), id, t);

                    let op = operator.0.value.as_ref().unwrap();
                    println!("{}AssignmentOperator ({})", "  ".repeat(layer + 1), op);

                    expression.display(layer + 1);
                }
                Let::TypedWithoutValue(identifier, type_identifier) => {
                    let id = identifier.0.value.as_ref().unwrap();
                    let t = type_identifier.0.value.as_ref().unwrap();
                    println!("{}LetStatement ({}) ({})", "  ".repeat(layer), id, t);
                }
            },
            Self::Block(b) => {
                println!("{}BlockStatement", "  ".repeat(layer));
                for statement in &b.0 {
                    statement.display(layer + 1)
                }
            }
            Self::Assignment(identifier, operator, expression) => {
                let id = identifier.0.value.as_ref().unwrap();
                println!("{}AssignmentStatement ({})", "  ".repeat(layer), id);

                let op = operator.0.value.as_ref().unwrap();
                println!("{}AssignmentOperator ({})", "  ".repeat(layer + 1), op);
                expression.display(layer + 1);
            }
            Self::Return(r) => match r {
                Return::WithExpression(expression) => {
                    println!("{}ReturnStatement", "  ".repeat(layer));
                    expression.display(layer + 1);
                }
                Return::WithoutExpression => {
                    println!("{}ReturnStatement", "  ".repeat(layer))
                }
            },
            Self::If(expression, b) => {
                println!("{}IfStatement", "  ".repeat(layer));
                expression.display(layer + 1);
                for statement in &b.0 {
                    statement.display(layer + 1)
                }
            }
            Self::FunctionCall(identifier, params) => {
                let id = identifier.0.value.as_ref().unwrap();
                let expressions = &params.0;
                println!("{}FunctionCallStatement ({})", "  ".repeat(layer), id);

                for expression in expressions {
                    expression.display(layer + 1);
                }
            }
        }
    }
}
