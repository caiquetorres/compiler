use crate::lang::position::Positioned;
use crate::lang::semantic::semantic_type::SemanticType;
use crate::lang::semantic::{scope::Scope, semantic_error::SemanticError};
use crate::lang::syntax::expressions::array::Array;

use std::{cell::RefCell, rc::Rc};

use super::expression_analyzer::ExpressionAnalyzer;

/// Analyzer that performs the semantic analysis for arrays.
pub struct ArrayAnalyzer {
    /// The inferred return type after semantic analyses.
    pub(crate) return_type: SemanticType,

    /// A collection of semantic errors found during analysis.
    pub(crate) diagnosis: Vec<SemanticError>,
}

impl ArrayAnalyzer {
    /// Analyzes the provided array within a given scope.
    /// # Arguments
    ///
    /// * `array` - A reference to the array to be analyzed.
    /// * `scope` - A reference-counted reference to the scope in which the analysis occurs.
    ///
    /// # Returns
    ///
    /// A `ArrayAnalyzer` instance containing the analysis results.
    pub fn analyze(array: &Array, scope: Rc<RefCell<Scope>>) -> Self {
        let return_type: SemanticType;
        let mut diagnosis: Vec<SemanticError> = vec![];

        // Verifies if the array is not empty
        if array.expressions.len() != 0 {
            // Considers the array type the same as its first element type.
            let first_array_expression = array.expressions.get(0).unwrap();
            let analyzer = ExpressionAnalyzer::analyze(first_array_expression, Rc::clone(&scope));
            let first_element_type = analyzer.return_type;

            diagnosis.extend(analyzer.diagnosis);

            // if let SemanticType::Function(_, _) = first_element_type {
            //     diagnosis.push(SemanticError::ArraysCannotHaveFunctions {
            //         position: first_array_expression.get_position(),
            //     });
            // }

            // REVIEW: Save the elements which the type is invalid.
            // REVIEW: Improve the following logic

            for (_, expression) in array.expressions.iter().enumerate() {
                let analyzer = ExpressionAnalyzer::analyze(expression, Rc::clone(&scope));

                diagnosis.extend(analyzer.diagnosis);

                let same_or_compatible_types = analyzer.return_type == first_element_type
                    || analyzer.return_type.is_number() && first_element_type.is_number();

                if !same_or_compatible_types {
                    diagnosis.push(SemanticError::InvalidArrayElement {
                        expected: first_element_type.clone(),
                        found: analyzer.return_type,
                        position: expression.get_position(),
                    });
                }
            }

            return_type =
                SemanticType::Array(Box::new(first_element_type), array.expressions.len());
        } else {
            // If the array is empty then its type is array of any.
            return_type = SemanticType::Array(Box::new(SemanticType::Any), 0);
        }

        Self {
            return_type,
            diagnosis,
        }
    }
}
