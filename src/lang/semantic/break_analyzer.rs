use std::{cell::RefCell, rc::Rc};

use crate::lang::syntax::parser::statements::r#break::Break;

use super::{scope::Scope, semantic_error::SemanticError};

pub struct BreakAnalyzer {
    pub(crate) diagnosis: Vec<SemanticError>,
}

impl BreakAnalyzer {
    pub fn analyze(_: &Break, scope: Rc<RefCell<Scope>>) -> Self {
        let mut diagnosis: Vec<SemanticError> = vec![];

        if !scope.borrow().is_loop() {
            diagnosis.push(SemanticError::InvalidBreak);
        }

        Self { diagnosis }
    }
}
