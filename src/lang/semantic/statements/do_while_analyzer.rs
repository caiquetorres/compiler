use std::{cell::RefCell, rc::Rc};

use crate::lang::syntax::statements::do_while::DoWhile;
use crate::lang::semantic::analyzer::Scopes;
use crate::lang::semantic::expressions::expression_analyzer::ExpressionAnalyzer;
use crate::lang::semantic::scope::Scope;
use crate::lang::semantic::semantic_error::SemanticError;
use crate::lang::semantic::semantic_type::SemanticType;

use super::block_analyzer::BlockAnalyzer;

/// Analyzer responsible for semantic analysis of 'do-while' loops.
pub struct DoWhileAnalyzer {
    /// A collection of semantic errors found during analysis.
    pub(crate) diagnosis: Vec<SemanticError>,
}

impl DoWhileAnalyzer {
    /// Analyzes the provided 'do-while' loop within a given scope.
    ///
    /// # Arguments
    ///
    /// * `do_while` - A reference to the 'do-while' loop to be analyzed.
    /// * `scope` - A reference-counted reference to the scope in which the analysis occurs.
    /// * `scopes` - A mutable reference to the set of scopes available for analysis.
    ///
    /// # Returns
    ///
    /// A `DoWhileAnalyzer` instance containing the analysis results.
    pub fn analyze(do_while: &DoWhile, scope: Rc<RefCell<Scope>>, scopes: &mut Scopes) -> Self {
        let mut diagnosis: Vec<SemanticError> = vec![];

        let scope = Rc::new(RefCell::new(Scope::new(Rc::clone(&scope), true, None)));

        let analyzer =
            BlockAnalyzer::analyze_within_scope(&do_while.block, Rc::clone(&scope), scopes);

        diagnosis.extend(analyzer.diagnosis);

        let analyzer = ExpressionAnalyzer::analyze(&do_while.expression, Rc::clone(&scope));

        diagnosis.extend(analyzer.diagnosis);

        if analyzer.return_type != SemanticType::Bool {
            diagnosis.push(SemanticError::ExpectedType {
                expected: SemanticType::Bool,
                found: analyzer.return_type,
            })
        }

        Self { diagnosis }
    }
}
