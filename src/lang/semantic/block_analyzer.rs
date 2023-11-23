use std::{cell::RefCell, rc::Rc};

use crate::lang::syntax::parser::{shared::block::Block, statements::statement::Statement};

use super::{
    analyzer::Scopes,
    assignment_analyzer::AssignmentAnalyzer,
    break_analyzer::BreakAnalyzer,
    continue_analyzer::ContinueAnalyzer,
    for_analyzer::ForAnalyzer,
    function_call_analyzer::FunctionCallAnalyzer,
    print_analyzer::PrintAnalyzer,
    return_analyzer::ReturnAnalyzer,
    scope::Scope,
    semantic_error::SemanticError,
    statements::{
        const_analyzer::ConstAnalyzer, do_while_analyzer::DoWhileAnalyzer, if_analyzer::IfAnalyzer,
        let_analyzer::LetAnalyzer, while_analyzer::WhileAnalyzer,
    },
};

pub struct BlockAnalyzer {
    pub(crate) diagnosis: Vec<SemanticError>,
}

impl BlockAnalyzer {
    pub fn analyze(block: &Block, parent_scope: Rc<RefCell<Scope>>, scopes: &mut Scopes) -> Self {
        let scope = Scope::new(parent_scope, false, None);
        Self::analyze_within_scope(block, Rc::new(RefCell::new(scope)), scopes)
    }

    pub fn analyze_within_scope(
        block: &Block,
        scope: Rc<RefCell<Scope>>,
        scopes: &mut Scopes,
    ) -> Self {
        let mut diagnosis: Vec<SemanticError> = vec![];

        for statement in &block.statements {
            let scope = Rc::clone(&scope);

            match statement {
                Statement::Block(block) => {
                    let analyzer = Self::analyze(block, scope, scopes);
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
                    let analyzer = WhileAnalyzer::analyze(r#while, scope, scopes);
                    diagnosis.extend(analyzer.diagnosis);
                }
                Statement::DoWhile(do_while) => {
                    let analyzer = DoWhileAnalyzer::analyze(do_while, scope, scopes);
                    diagnosis.extend(analyzer.diagnosis);
                }
                Statement::For(r#for) => {
                    let analyzer = ForAnalyzer::analyze(r#for, scope, scopes);
                    diagnosis.extend(analyzer.diagnosis);
                }
                Statement::If(r#if) => {
                    let analyzer = IfAnalyzer::analyze(r#if, scope, scopes);
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

        scopes.insert(block.id, Rc::clone(&scope));

        Self { diagnosis }
    }
}
