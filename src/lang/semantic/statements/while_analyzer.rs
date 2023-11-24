use std::{cell::RefCell, rc::Rc};

use crate::lang::semantic::analyzer::Scopes;
use crate::lang::semantic::expressions::expression_analyzer::ExpressionAnalyzer;
use crate::lang::semantic::scope::Scope;
use crate::lang::semantic::semantic_error::SemanticError;
use crate::lang::semantic::semantic_type::SemanticType;
use crate::lang::syntax::parser::statements::r#while::While;

use super::block_analyzer::BlockAnalyzer;

/// Analyzer responsible for semantic analysis of 'while' loops.
pub struct WhileAnalyzer {
    /// A collection of semantic errors found during analysis.
    pub(crate) diagnosis: Vec<SemanticError>,
}

impl WhileAnalyzer {
    /// Analyzes the provided 'while' loop within a given scope.
    ///
    /// # Arguments
    ///
    /// * `r#while` - A reference to the 'while' loop to be analyzed.
    /// * `scope` - A reference-counted reference to the scope in which the analysis occurs.
    /// * `scopes` - A mutable reference to the set of scopes available for analysis.
    ///
    /// # Returns
    ///
    /// A `WhileAnalyzer` instance containing the analysis results.
    pub fn analyze(r#while: &While, scope: Rc<RefCell<Scope>>, scopes: &mut Scopes) -> Self {
        let mut diagnosis: Vec<SemanticError> = vec![];

        let scope = Rc::new(RefCell::new(Scope::new(Rc::clone(&scope), true, None)));

        let analyzer = ExpressionAnalyzer::analyze(&r#while.expression, Rc::clone(&scope));

        if !analyzer.return_type.is_bool() {
            diagnosis.push(SemanticError::ExpectedType {
                expected: SemanticType::Bool,
                found: analyzer.return_type,
            })
        }

        let analyzer =
            BlockAnalyzer::analyze_within_scope(&r#while.block, Rc::clone(&scope), scopes);

        diagnosis.extend(analyzer.diagnosis);

        Self { diagnosis }
    }
}
