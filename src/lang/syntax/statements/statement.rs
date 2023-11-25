use super::assignment::Assignment;
use super::do_while::DoWhile;
use super::print::Print;
use super::r#break::Break;
use super::r#continue::Continue;
use super::r#for::For;
use super::r#if::If;
use super::r#let::Let;
use super::r#return::Return;
use super::r#while::While;

use crate::lang::syntax::expressions::expression::Expression;
use crate::lang::syntax::shared::block::Block;
use crate::lang::syntax::tree_display::TreeDisplay;

#[derive(Clone, Debug)]
pub enum Statement {
    Let(Let),
    Block(Block),
    Return(Return),
    If(If),
    While(While),
    DoWhile(DoWhile),
    For(For),
    Break(Break),
    Continue(Continue),
    Print(Print),
    Assignment(Assignment),
    Expression(Expression),
}

impl TreeDisplay for Statement {
    fn display(&self, layer: usize) {
        match &self {
            Self::Expression(expression) => expression.display(layer),
            Self::Let(r#let) => r#let.display(layer),
            Self::Block(block) => block.display(layer),
            Self::Assignment(assignment) => assignment.display(layer),
            Self::Return(r#return) => r#return.display(layer),
            Self::If(r#if) => r#if.display(layer),
            Self::While(r#while) => r#while.display(layer),
            Self::DoWhile(do_while) => do_while.display(layer),
            Self::For(r#for) => r#for.display(layer),
            Self::Break(r#break) => r#break.display(layer),
            Self::Continue(r#continue) => r#continue.display(layer),
            Self::Print(print) => print.display(layer),
        }
    }
}
