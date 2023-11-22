use std::{cell::RefCell, rc::Rc};

use crate::lang::syntax::parser::statements::do_while::DoWhile;

use super::{
    analyzer::Scopes, block_analyzer::BlockAnalyzer, expression_analyzer::ExpressionAnalyzer,
    lang_type::LangType, scope::Scope, semantic_error::SemanticError,
};

pub struct DoWhileAnalyzer {
    pub(crate) diagnosis: Vec<SemanticError>,
}

impl DoWhileAnalyzer {
    pub fn analyze(do_while: &DoWhile, scope: Rc<RefCell<Scope>>, scopes: &mut Scopes) -> Self {
        let mut diagnosis: Vec<SemanticError> = vec![];

        let scope = Rc::new(RefCell::new(Scope::new(Rc::clone(&scope), true, None)));

        let analyzer =
            BlockAnalyzer::analyze_within_scope(&do_while.block, Rc::clone(&scope), scopes);

        diagnosis.extend(analyzer.diagnosis);

        let analyzer = ExpressionAnalyzer::analyze(&do_while.expression, Rc::clone(&scope));

        if analyzer.return_type != LangType::Bool {
            diagnosis.push(SemanticError::ExpectedType {
                expected: LangType::Bool,
                found: analyzer.return_type,
            })
        }

        Self { diagnosis }
    }
}
