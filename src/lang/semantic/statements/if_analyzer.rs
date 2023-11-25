use std::{cell::RefCell, rc::Rc};

use crate::lang::syntax::statements::r#if::If;
use crate::lang::semantic::analyzer::Scopes;
use crate::lang::semantic::expressions::expression_analyzer::ExpressionAnalyzer;
use crate::lang::semantic::scope::Scope;
use crate::lang::semantic::semantic_error::SemanticError;
use crate::lang::semantic::semantic_type::SemanticType;

use super::block_analyzer::BlockAnalyzer;

/// Analyzer responsible for semantic analysis of 'if' statements.
pub struct IfAnalyzer {
    /// A collection of semantic errors found during analysis.
    pub(crate) diagnosis: Vec<SemanticError>,
}

impl IfAnalyzer {
    /// Analyzes the provided 'if' statement within a given scope.
    ///
    /// # Arguments
    ///
    /// * `r#if` - A reference to the 'if' statement to be analyzed.
    /// * `scope` - A reference-counted reference to the scope in which the analysis occurs.
    /// * `scopes` - A mutable reference to the set of scopes available for analysis.
    ///
    /// # Returns
    ///
    /// An `IfAnalyzer` instance containing the analysis results.
    pub fn analyze(r#if: &If, scope: Rc<RefCell<Scope>>, scopes: &mut Scopes) -> Self {
        let mut diagnosis: Vec<SemanticError> = vec![];

        let analyzer = ExpressionAnalyzer::analyze(&r#if.expression, Rc::clone(&scope));

        diagnosis.extend(analyzer.diagnosis);

        if !analyzer.return_type.is_bool() {
            diagnosis.push(SemanticError::ExpectedType {
                expected: SemanticType::Bool,
                found: analyzer.return_type,
            })
        }

        let analyzer = BlockAnalyzer::analyze(&r#if.block, Rc::clone(&scope), scopes);

        diagnosis.extend(analyzer.diagnosis);

        if let Some(r#else) = &r#if.r#else {
            let analyzer = BlockAnalyzer::analyze(&r#else.block, Rc::clone(&scope), scopes);

            diagnosis.extend(analyzer.diagnosis);
        }

        Self { diagnosis }
    }
}
