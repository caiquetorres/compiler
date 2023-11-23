use crate::lang::semantic::expression_analyzer::ExpressionAnalyzer;
use crate::lang::semantic::scope::Scope;
use crate::lang::semantic::semantic_error::SemanticError;
use crate::lang::semantic::semantic_type::SemanticType;
use crate::lang::syntax::parser::statements::r#return::Return;

use std::{cell::RefCell, rc::Rc};

/// Analyzer responsible for semantic analysis of 'return' statements.
pub struct ReturnAnalyzer {
    /// A collection of semantic errors found during analysis.
    pub(crate) diagnosis: Vec<SemanticError>,
}

impl ReturnAnalyzer {
    /// Analyzes the 'return' statement within a given scope.
    ///
    /// # Arguments
    ///
    /// * `r#return` - A reference to the 'return' statement to be analyzed.
    /// * `scope` - A reference-counted reference to the scope in which the analysis occurs.
    ///
    /// # Returns
    ///
    /// A `ReturnAnalyzer` instance containing the analysis results.
    pub fn analyze(r#return: &Return, scope: Rc<RefCell<Scope>>) -> Self {
        let mut diagnosis: Vec<SemanticError> = vec![];

        if let Some(function_return_type) = scope.borrow().get_return_type() {
            let return_type = match &r#return.expression {
                None => SemanticType::Void,
                Some(expression) => {
                    let analyzer = ExpressionAnalyzer::analyze(expression, Rc::clone(&scope));

                    diagnosis.extend(analyzer.diagnosis);

                    analyzer.return_type
                }
            };

            if function_return_type != return_type
                && (!function_return_type.is_number() || !return_type.is_number())
            {
                diagnosis.push(SemanticError::ExpectedType {
                    expected: function_return_type,
                    found: return_type,
                })
            }
        } else {
            diagnosis.push(SemanticError::InvalidReturn)
        }

        Self { diagnosis }
    }
}
