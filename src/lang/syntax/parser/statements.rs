use super::expressions::Expression;
use crate::lang::syntax::{lexer::token::Token, tree_display::TreeDisplay};

pub struct Identifier(pub Token);

pub struct AssignmentOperator(pub Token);

pub struct Block(pub Vec<Statement>);

pub enum Let {
    WithValue(
        Identifier,
        Option<Identifier>,
        AssignmentOperator,
        Expression,
    ),
    WithoutValue(Identifier, Identifier),
}

pub enum Const {
    WithValue(
        Identifier,
        Option<Identifier>,
        AssignmentOperator,
        Expression,
    ),
    WithoutValue(Identifier, Identifier),
}

pub struct ParamDeclaration(pub Identifier, pub Identifier);

pub struct ParamsDeclaration(pub Vec<ParamDeclaration>);

pub struct Params(pub Vec<Expression>);

pub enum TopLevelStatement {
    Function(Identifier, ParamsDeclaration, Option<Identifier>, Block),
}

impl TreeDisplay for ParamDeclaration {
    fn display(&self, layer: usize) {
        let param_name = self.0 .0.value.as_ref().unwrap();
        let param_type = self.1 .0.value.as_ref().unwrap();

        println!(
            "{}ParamDeclaration ({}) ({})",
            "  ".repeat(layer + 1),
            param_name,
            param_type
        );
    }
}

impl TreeDisplay for TopLevelStatement {
    fn display(&self, layer: usize) {
        match self {
            Self::Function(id, params, t, block) => {
                display_function_statement(layer, id, params, t, block)
            }
        }
    }
}

pub enum Statement {
    Let(Let),
    Const(Const),
    Block(Block),
    Assignment(Identifier, AssignmentOperator, Expression),
    Return(Option<Expression>),
    If(Expression, Box<Statement>, Option<ElseStatement>),
    FunctionCall(Identifier, Params),
    While(Expression, Box<Statement>),
    DoWhile(Box<Statement>, Expression),
    For(Identifier, Expression, Box<Statement>),
}

pub struct ElseStatement(pub Box<Statement>);

impl TreeDisplay for Statement {
    fn display(&self, layer: usize) {
        match self {
            Self::For(id, expression, b) => {
                println!(
                    "{}ForStatement ({})",
                    "  ".repeat(layer),
                    id.0.value.as_ref().unwrap()
                );
                expression.display(layer + 1);
                b.display(layer + 1);
            }
            Self::DoWhile(b, expression) => {
                println!("{}DoWhileStatement", "  ".repeat(layer));
                b.display(layer + 1);
                expression.display(layer + 1);
            }
            Self::While(expression, b) => {
                println!("{}WhileStatement", "  ".repeat(layer));
                expression.display(layer + 1);
                b.display(layer + 1);
            }
            Self::Const(c) => match c {
                Const::WithValue(identifier, opt_type, operator, expression) => {
                    display_const_with_value_statement(
                        layer, identifier, opt_type, operator, expression,
                    )
                }
                Const::WithoutValue(identifier, type_identifier) => {
                    let id = identifier.0.value.as_ref().unwrap();
                    let t = type_identifier.0.value.as_ref().unwrap();
                    println!("{}ConstStatement ({}) ({})", "  ".repeat(layer), id, t);
                }
            },
            Self::Let(l) => match l {
                Let::WithValue(identifier, opt_type, operator, expression) => {
                    display_let_with_value_statement(
                        layer, identifier, opt_type, operator, expression,
                    )
                }
                Let::WithoutValue(identifier, type_identifier) => {
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
            Self::Return(expression) => {
                println!("{}ReturnStatement", "  ".repeat(layer));
                if let Some(ex) = expression {
                    ex.display(layer + 1)
                }
            }
            Self::If(expression, b, else_statement) => {
                println!("{}IfStatement", "  ".repeat(layer));
                expression.display(layer + 1);
                b.display(layer + 1);
                println!("{}ElseStatement", "  ".repeat(layer));

                if let Some(st) = else_statement {
                    st.0.display(layer + 1)
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

fn display_function_statement(
    layer: usize,
    identifier: &Identifier,
    params: &ParamsDeclaration,
    return_type: &Option<Identifier>,
    block: &Block,
) {
    match return_type {
        Some(id) => {
            println!(
                "{}FunctionDeclaration ({}) ({})",
                "  ".repeat(layer),
                identifier.0.value.as_ref().unwrap(),
                id.0.value.as_ref().unwrap()
            );
        }
        None => {
            println!(
                "{}FunctionDeclaration ({})",
                "  ".repeat(layer),
                identifier.0.value.as_ref().unwrap(),
            );
        }
    }

    for param in &params.0 {
        param.display(layer);
    }

    let statements = &block.0;
    for statement in statements {
        statement.display(layer + 1);
    }
}

fn display_let_with_value_statement(
    layer: usize,
    identifier: &Identifier,
    opt_type: &Option<Identifier>,
    operator: &AssignmentOperator,
    expression: &Expression,
) {
    let id = identifier.0.value.as_ref().unwrap();

    match opt_type {
        None => {
            println!("{}LetStatement ({})", "  ".repeat(layer), id);
        }
        Some(t) => {
            println!(
                "{}LetStatement ({}) ({})",
                "  ".repeat(layer),
                id,
                t.0.value.as_ref().unwrap()
            );
        }
    }

    let op = operator.0.value.as_ref().unwrap();
    println!("{}AssignmentOperator ({})", "  ".repeat(layer + 1), op);

    expression.display(layer + 1);
}

fn display_const_with_value_statement(
    layer: usize,
    identifier: &Identifier,
    opt_type: &Option<Identifier>,
    operator: &AssignmentOperator,
    expression: &Expression,
) {
    let id = identifier.0.value.as_ref().unwrap();

    match opt_type {
        None => {
            println!("{}ConstStatement ({})", "  ".repeat(layer), id);
        }
        Some(t) => {
            println!(
                "{}ConstStatement ({}) ({})",
                "  ".repeat(layer),
                id,
                t.0.value.as_ref().unwrap()
            );
        }
    }

    let op = operator.0.value.as_ref().unwrap();
    println!("{}AssignmentOperator ({})", "  ".repeat(layer + 1), op);

    expression.display(layer + 1);
}
