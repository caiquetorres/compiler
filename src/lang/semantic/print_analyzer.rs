use std::{cell::RefCell, rc::Rc};

use crate::lang::syntax::parser::statements::print::Print;

use super::{expression_analyzer::ExpressionAnalyzer, scope::Scope, semantic_error::SemanticError};

pub struct PrintAnalyzer {
    pub(crate) diagnosis: Vec<SemanticError>,
}

impl PrintAnalyzer {
    pub fn analyze(print: &Print, scope: Rc<RefCell<Scope>>) -> Self {
        let mut diagnosis: Vec<SemanticError> = vec![];

        for expression in &print.expressions {
            let analyzer = ExpressionAnalyzer::analyze(expression, Rc::clone(&scope));
            diagnosis.extend(analyzer.diagnosis);
        }

        Self { diagnosis }
    }
}
