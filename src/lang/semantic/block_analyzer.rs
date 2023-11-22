use std::{cell::RefCell, rc::Rc};

use crate::lang::syntax::parser::{shared::block::Block, statements::statement::Statement};

use super::{
    const_analyzer::ConstAnalyzer, let_analyzer::LetAnalyzer, scope::Scope,
    semantic_error::SemanticError,
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
                _ => {}
            }
        }

        Self { diagnosis }
    }
}
