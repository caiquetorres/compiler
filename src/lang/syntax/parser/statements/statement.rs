use super::assignment::Assignment;
use super::do_while::DoWhile;
use super::r#break::Break;
use super::r#const::Const;
use super::r#continue::Continue;
use super::r#for::For;
use super::r#if::If;
use super::r#let::Let;
use super::r#return::Return;
use super::r#while::While;

use crate::lang::syntax::parser::shared::block::Block;
use crate::lang::syntax::parser::shared::function_call::FunctionCall;
use crate::lang::syntax::tree_display::TreeDisplay;

pub enum Statement {
    Let(Let),
    Const(Const),
    Block(Block),
    Assignment(Assignment),
    Return(Return),
    If(If),
    FunctionCall(FunctionCall),
    While(While),
    DoWhile(DoWhile),
    For(For),
    Break(Break),
    Continue(Continue),
}

impl TreeDisplay for Statement {
    fn display(&self, layer: usize) {
        match &self {
            Self::Let(r#let) => r#let.display(layer),
            Self::Const(r#const) => r#const.display(layer),
            Self::Block(block) => block.display(layer),
            Self::Assignment(assignment) => assignment.display(layer),
            Self::Return(r#return) => r#return.display(layer),
            Self::If(r#if) => r#if.display(layer),
            Self::FunctionCall(call) => call.display(layer),
            Self::While(r#while) => r#while.display(layer),
            Self::DoWhile(do_while) => do_while.display(layer),
            Self::For(r#for) => r#for.display(layer),
            Self::Break(r#break) => r#break.display(layer),
            Self::Continue(r#continue) => r#continue.display(layer),
        }
    }
}
