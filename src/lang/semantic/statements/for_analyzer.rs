use crate::lang::semantic::analyzer::Scopes;
use crate::lang::semantic::expression_analyzer::ExpressionAnalyzer;
use crate::lang::semantic::scope::Scope;
use crate::lang::semantic::semantic_error::SemanticError;
use crate::lang::semantic::semantic_type::SemanticType;
use crate::lang::semantic::symbol::Symbol;
use crate::lang::syntax::parser::expressions::expression::Expression;
use crate::lang::syntax::parser::statements::r#for::For;

use std::{cell::RefCell, rc::Rc};

use super::block_analyzer::BlockAnalyzer;

/// Analyzer responsible for semantic analysis of 'for' loops.
pub struct ForAnalyzer {
    /// A collection of semantic errors found during analysis.
    pub(crate) diagnosis: Vec<SemanticError>,
}

impl ForAnalyzer {
    /// Analyzes the provided 'for' loop within a given scope.
    ///
    /// # Arguments
    ///
    /// * `r#for` - A reference to the 'for' loop to be analyzed.
    /// * `scope` - A reference-counted reference to the scope in which the analysis occurs.
    /// * `scopes` - A mutable reference to the set of scopes available for analysis.
    ///
    /// # Returns
    ///
    /// A `ForAnalyzer` instance containing the analysis results.
    pub fn analyze(r#for: &For, scope: Rc<RefCell<Scope>>, scopes: &mut Scopes) -> Self {
        let mut diagnosis: Vec<SemanticError> = vec![];

        let scope = Rc::new(RefCell::new(Scope::new(Rc::clone(&scope), true, None)));

        let identifier_name = r#for.identifier.name.clone();

        if let Some(_) = scope.borrow().get(&identifier_name) {
            diagnosis.push(SemanticError::DuplicatedIdentifier);
        }

        let identifier_type = SemanticType::Any;

        if let Expression::Range(range) = &r#for.expression {
            let left_analyzer = ExpressionAnalyzer::analyze(&range.left, Rc::clone(&scope));
            diagnosis.extend(left_analyzer.diagnosis);

            let right_analyzer = ExpressionAnalyzer::analyze(&range.right, Rc::clone(&scope));
            diagnosis.extend(right_analyzer.diagnosis);

            // Verifies whether the both types are numbers or not.
            if !right_analyzer.return_type.is_number() || !left_analyzer.return_type.is_number() {
                diagnosis.push(SemanticError::InvalidRangeOperands);
            }

            // REVIEW: Should web check the type? In order to ensure that the types are both integers?

            scope.borrow_mut().insert(Symbol::Const {
                name: identifier_name,
                symbol_type: SemanticType::number_type_precedence(vec![
                    left_analyzer.return_type,
                    right_analyzer.return_type,
                ]),
            });
        } else {
            // If the expression is not a range expression we still verifies its types, but we keep the type Any;

            let analyzer = ExpressionAnalyzer::analyze(&r#for.expression, Rc::clone(&scope));
            diagnosis.extend(analyzer.diagnosis);

            diagnosis.push(SemanticError::ExpectedType {
                expected: SemanticType::Range,
                found: analyzer.return_type,
            });

            scope.borrow_mut().insert(Symbol::Const {
                name: identifier_name,
                symbol_type: identifier_type,
            });
        }

        let analyzer = BlockAnalyzer::analyze_within_scope(&r#for.block, Rc::clone(&scope), scopes);
        diagnosis.extend(analyzer.diagnosis);

        Self { diagnosis }
    }
}
