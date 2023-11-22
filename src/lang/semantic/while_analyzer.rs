use std::{cell::RefCell, rc::Rc};

use crate::lang::syntax::parser::statements::r#while::While;

use super::{
    analyzer::Scopes, block_analyzer::BlockAnalyzer, expression_analyzer::ExpressionAnalyzer,
    lang_type::LangType, scope::Scope, semantic_error::SemanticError,
};

pub struct WhileAnalyzer {
    pub(crate) diagnosis: Vec<SemanticError>,
}

impl WhileAnalyzer {
    pub fn analyze(r#while: &While, scope: Rc<RefCell<Scope>>, scopes: &mut Scopes) -> Self {
        let mut diagnosis: Vec<SemanticError> = vec![];

        let scope = Rc::new(RefCell::new(Scope::new(Rc::clone(&scope), true, None)));

        let analyzer = ExpressionAnalyzer::analyze(&r#while.expression, Rc::clone(&scope));

        if analyzer.return_type != LangType::Bool {
            diagnosis.push(SemanticError::ExpectedType {
                expected: LangType::Bool,
                found: analyzer.return_type,
            })
        }

        let analyzer =
            BlockAnalyzer::analyze_within_scope(&r#while.block, Rc::clone(&scope), scopes);

        diagnosis.extend(analyzer.diagnosis);

        Self { diagnosis }
    }
}
