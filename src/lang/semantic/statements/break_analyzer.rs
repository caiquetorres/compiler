use crate::lang::semantic::semantic_error::SemanticError;
use crate::lang::syntax::statements::r#break::Break;
use crate::lang::{position::Positioned, semantic::scope::Scope};

use std::{cell::RefCell, rc::Rc};

/// Analyzer responsible for semantic analysis of 'break' statements.
pub struct BreakAnalyzer {
    /// A collection of semantic errors found during analysis.
    pub(crate) diagnosis: Vec<SemanticError>,
}

impl BreakAnalyzer {
    /// Analyzes the 'break' statement within a given scope.
    ///
    /// # Arguments
    ///
    /// * `_` - Represents the 'break' statement to be analyzed (unused in analysis).
    /// * `scope` - A reference-counted reference to the scope in which the analysis occurs.
    ///
    /// # Returns
    ///
    /// A `BreakAnalyzer` instance containing the analysis results.
    pub fn analyze(r#break: &Break, scope: Rc<RefCell<Scope>>) -> Self {
        let mut diagnosis: Vec<SemanticError> = vec![];

        if !scope.borrow().is_loop() {
            diagnosis.push(SemanticError::InvalidBreak {
                position: r#break.get_position(),
            });
        }

        Self { diagnosis }
    }
}
