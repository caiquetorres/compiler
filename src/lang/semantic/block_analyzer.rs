use std::{cell::RefCell, rc::Rc};

use crate::lang::syntax::parser::{shared::block::Block, statements::statement::Statement};

use super::{
    assignment_analyzer::AssignmentAnalyzer, break_analyzer::BreakAnalyzer,
    const_analyzer::ConstAnalyzer, continue_analyzer::ContinueAnalyzer,
    do_while_analyzer::DoWhileAnalyzer, for_analyzer::ForAnalyzer,
    function_call_analyzer::FunctionCallAnalyzer, if_analyzer::IfAnalyzer,
    let_analyzer::LetAnalyzer, print_analyzer::PrintAnalyzer, return_analyzer::ReturnAnalyzer,
    scope::Scope, semantic_error::SemanticError, while_analyzer::WhileAnalyzer,
};

pub struct BlockAnalyzer {
    pub(crate) diagnosis: Vec<SemanticError>,
}

impl BlockAnalyzer {
    pub fn analyze(block: &Block, parent_scope: Rc<RefCell<Scope>>) -> Self {
        let scope = Scope::new(parent_scope, false, None);
        Self::analyze_within_scope(block, Rc::new(RefCell::new(scope)))
    }

    pub fn analyze_within_scope(block: &Block, scope: Rc<RefCell<Scope>>) -> Self {
        let mut diagnosis: Vec<SemanticError> = vec![];

        for statement in &block.statements {
            let scope = Rc::clone(&scope);

            match statement {
                Statement::Block(block) => {
                    let analyzer = Self::analyze(block, scope);
                    diagnosis.extend(analyzer.diagnosis);
                }
                Statement::Let(r#let) => {
                    let analyzer = LetAnalyzer::analyze(r#let, scope);
                    diagnosis.extend(analyzer.diagnosis);
                }
                Statement::Const(r#const) => {
                    let analyzer = ConstAnalyzer::analyze(r#const, scope);
                    diagnosis.extend(analyzer.diagnosis);
                }
                Statement::Assignment(assignment) => {
                    let analyzer = AssignmentAnalyzer::analyze(assignment, scope);
                    diagnosis.extend(analyzer.diagnosis);
                }
                Statement::While(r#while) => {
                    let analyzer = WhileAnalyzer::analyze(r#while, scope);
                    diagnosis.extend(analyzer.diagnosis);
                }
                Statement::DoWhile(do_while) => {
                    let analyzer = DoWhileAnalyzer::analyze(do_while, scope);
                    diagnosis.extend(analyzer.diagnosis);
                }
                Statement::For(r#for) => {
                    let analyzer = ForAnalyzer::analyze(r#for, scope);
                    diagnosis.extend(analyzer.diagnosis);
                }
                Statement::If(r#if) => {
                    let analyzer = IfAnalyzer::analyze(r#if, scope);
                    diagnosis.extend(analyzer.diagnosis);
                }
                Statement::Break(r#break) => {
                    let analyzer = BreakAnalyzer::analyze(r#break, scope);
                    diagnosis.extend(analyzer.diagnosis);
                }
                Statement::Continue(r#continue) => {
                    let analyzer = ContinueAnalyzer::analyze(r#continue, scope);
                    diagnosis.extend(analyzer.diagnosis);
                }
                Statement::Return(r#return) => {
                    let analyzer = ReturnAnalyzer::analyze(r#return, scope);
                    diagnosis.extend(analyzer.diagnosis);
                }
                Statement::FunctionCall(function_call) => {
                    let analyzer = FunctionCallAnalyzer::analyze(function_call, scope);
                    diagnosis.extend(analyzer.diagnosis);
                }
                Statement::Print(print) => {
                    let analyzer = PrintAnalyzer::analyze(print, scope);
                    diagnosis.extend(analyzer.diagnosis);
                }
            }
        }

        Self { diagnosis }
    }
}
