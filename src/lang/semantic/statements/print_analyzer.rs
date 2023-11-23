use crate::lang::semantic::expression_analyzer::ExpressionAnalyzer;
use crate::lang::semantic::scope::Scope;
use crate::lang::semantic::semantic_error::SemanticError;
use crate::lang::syntax::parser::statements::print::Print;

use std::{cell::RefCell, rc::Rc};

/// Analyzer responsible for semantic analysis of 'print' statements.
pub struct PrintAnalyzer {
    /// A collection of semantic errors found during analysis.
    pub(crate) diagnosis: Vec<SemanticError>,
}

impl PrintAnalyzer {
    /// Analyzes the 'print' statement within a given scope.
    ///
    /// # Arguments
    ///
    /// * `print` - A reference to the 'print' statement to be analyzed.
    /// * `scope` - A reference-counted reference to the scope in which the analysis occurs.
    ///
    /// # Returns
    ///
    /// A `PrintAnalyzer` instance containing the analysis results.
    pub fn analyze(print: &Print, scope: Rc<RefCell<Scope>>) -> Self {
        let mut diagnosis: Vec<SemanticError> = vec![];

        for expression in &print.expressions {
            let analyzer = ExpressionAnalyzer::analyze(expression, Rc::clone(&scope));
            diagnosis.extend(analyzer.diagnosis);
        }

        Self { diagnosis }
    }
}
