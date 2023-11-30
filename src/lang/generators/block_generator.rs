use std::rc::Rc;

use crate::lang::{
    semantic::analyzer::Scopes,
    syntax::{shared::block::Block, statements::statement::Statement},
};

use super::{
    c_code_generator2::CCode, expression_generator::ExpressionGenerator,
    for_generator::ForGenerator, let_generator::LetGenerator, print_generator::PrintGenerator,
};

pub struct BlockGenerator;

impl BlockGenerator {
    pub fn generate(block: &Block, scopes: &Scopes, ccode: &mut CCode) {
        let scope = scopes.get(&block.id).unwrap().clone();

        ccode.push("{");

        for statement in &block.statements {
            match statement {
                Statement::Expression(expression) => {
                    let code = ExpressionGenerator::generate(expression, Rc::clone(&scope), ccode);
                    ccode.push(&code);
                    ccode.push(";");
                }
                Statement::For(r#for) => ForGenerator::generate(r#for, scopes, ccode),
                Statement::Block(block) => Self::generate(block, scopes, ccode),
                Statement::Let(r#let) => LetGenerator::generate(r#let, Rc::clone(&scope), ccode),
                Statement::Print(print) => {
                    PrintGenerator::generate(print, Rc::clone(&scope), ccode)
                }
                Statement::Break(_) => {
                    ccode.push("break;");
                }
                Statement::Continue(_) => {
                    ccode.push("continue;");
                }
                Statement::DoWhile(do_while) => {
                    ccode.push("do");
                    Self::generate(&do_while.block, scopes, ccode);
                    ccode.push("while(");
                    let code = ExpressionGenerator::generate(
                        &do_while.expression,
                        Rc::clone(&scope),
                        ccode,
                    );
                    ccode.push(&code);
                    ccode.push(");");
                }
                Statement::While(r#while) => {
                    ccode.push("while(");

                    let code = ExpressionGenerator::generate(
                        &r#while.expression,
                        Rc::clone(&scope),
                        ccode,
                    );
                    ccode.push(&code);

                    ccode.push(")");
                    Self::generate(&r#while.block, scopes, ccode);
                }
                Statement::If(r#if) => {
                    ccode.push("if(");

                    let code =
                        ExpressionGenerator::generate(&r#if.expression, Rc::clone(&scope), ccode);
                    ccode.push(&code);
                    ccode.push(")");

                    Self::generate(&r#if.block, scopes, ccode);

                    match &r#if.r#else {
                        None => {}
                        Some(r#else) => {
                            ccode.push("else");
                            Self::generate(&r#else.block, scopes, ccode);
                        }
                    }
                }
                Statement::Assignment(assignment) => {
                    let left_code =
                        ExpressionGenerator::generate(&assignment.left, Rc::clone(&scope), ccode);

                    let right_code =
                        ExpressionGenerator::generate(&assignment.right, Rc::clone(&scope), ccode);

                    ccode.push(&format!(
                        "{}{}{};",
                        left_code, assignment.operator.name, right_code
                    ));
                }
                Statement::Return(r#return) => {
                    if scope.borrow().get_function_name().unwrap() == "main" {
                        ccode.push("return 0;");
                    } else {
                        match &r#return.expression {
                            None => ccode.push("return;"),
                            Some(expression) => {
                                ccode.push("return ");

                                let code = ExpressionGenerator::generate(
                                    expression,
                                    Rc::clone(&scope),
                                    ccode,
                                );

                                ccode.push(&code);
                                ccode.push(";");
                            }
                        }
                    }
                }
            }
        }

        ccode.push("}");
    }
}
