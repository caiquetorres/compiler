use std::{cell::RefCell, rc::Rc};

use crate::lang::position::Positioned;
use crate::lang::semantic::scope::Scope;
use crate::lang::semantic::semantic_error::SemanticError;
use crate::lang::syntax::statements::r#continue::Continue;

/// Analyzer responsible for semantic analysis of 'continue' statements.
pub struct ContinueAnalyzer {
    /// A collection of semantic errors found during analysis.
    pub(crate) diagnosis: Vec<SemanticError>,
}

impl ContinueAnalyzer {
    /// Analyzes the 'continue' statement within a given scope.
    ///
    /// # Arguments
    ///
    /// * `_` - Represents the 'continue' statement to be analyzed (unused in analysis).
    /// * `scope` - A reference-counted reference to the scope in which the analysis occurs.
    ///
    /// # Returns
    ///
    /// A `ContinueAnalyzer` instance containing the analysis results.
    pub fn analyze(r#continue: &Continue, scope: Rc<RefCell<Scope>>) -> Self {
        let mut diagnosis: Vec<SemanticError> = vec![];

        if !scope.borrow().is_loop() {
            diagnosis.push(SemanticError::InvalidContinue {
                position: r#continue.get_position(),
            });
        }

        Self { diagnosis }
    }
}
