use std::{cell::RefCell, rc::Rc};

use crate::lang::syntax::parser::statements::r#if::If;

use super::{
    block_analyzer::BlockAnalyzer, expression_analyzer::ExpressionAnalyzer, lang_type::LangType,
    scope::Scope, semantic_error::SemanticError,
};

pub struct IfAnalyzer {
    pub(crate) diagnosis: Vec<SemanticError>,
}

impl IfAnalyzer {
    pub fn analyze(r#if: &If, scope: Rc<RefCell<Scope>>) -> Self {
        let mut diagnosis: Vec<SemanticError> = vec![];

        let analyzer = ExpressionAnalyzer::analyze(&r#if.expression, Rc::clone(&scope));

        if analyzer.return_type != LangType::Bool {
            diagnosis.push(SemanticError::ExpectedType {
                expected: LangType::Bool,
                found: analyzer.return_type,
            })
        }

        let analyzer = BlockAnalyzer::analyze_within_scope(&r#if.block, Rc::clone(&scope));

        diagnosis.extend(analyzer.diagnosis);

        Self { diagnosis }
    }
}
