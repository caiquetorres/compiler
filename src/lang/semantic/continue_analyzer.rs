use std::{cell::RefCell, rc::Rc};

use crate::lang::syntax::parser::statements::r#continue::Continue;

use super::{scope::Scope, semantic_error::SemanticError};

pub struct ContinueAnalyzer {
    pub(crate) diagnosis: Vec<SemanticError>,
}

impl ContinueAnalyzer {
    pub fn analyze(_: &Continue, scope: Rc<RefCell<Scope>>) -> Self {
        let mut diagnosis: Vec<SemanticError> = vec![];

        if !scope.borrow().is_loop() {
            diagnosis.push(SemanticError::InvalidContinue);
        }

        Self { diagnosis }
    }
}
