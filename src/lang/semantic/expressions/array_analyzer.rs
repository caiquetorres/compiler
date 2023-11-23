use crate::lang::semantic::expression_analyzer::ExpressionAnalyzer;
use crate::lang::semantic::lang_type::LangType;
use crate::lang::semantic::scope::Scope;
use crate::lang::semantic::semantic_error::SemanticError;
use crate::lang::syntax::parser::expressions::array::Array;

use std::{cell::RefCell, rc::Rc};

/// Analyzer that performs the semantic analysis for arrays.
pub struct ArrayAnalyzer {
    /// The inferred return type after semantic analyses.
    pub return_type: LangType,

    /// A collection of semantic errors found during analysis.
    pub diagnosis: Vec<SemanticError>,
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
        let return_type: LangType;
        let mut diagnosis: Vec<SemanticError> = vec![];

        // Verifies if the array is not empty
        if array.expressions.len() != 0 {
            // Considers the array type the same as its first element type.
            let first_array_expression = array.expressions.get(0).unwrap();
            let analyzer = ExpressionAnalyzer::analyze(first_array_expression, Rc::clone(&scope));
            let first_element_type = analyzer.return_type;

            diagnosis.extend(analyzer.diagnosis);

            // REVIEW: Save the elements which the type is invalid.
            // REVIEW: Improve the following logic

            let mut all_same = true;
            for (_, element) in array.expressions.iter().enumerate() {
                let analyzer = ExpressionAnalyzer::analyze(element, Rc::clone(&scope));

                diagnosis.extend(analyzer.diagnosis);

                let same_or_compatible_types = analyzer.return_type == first_element_type
                    || analyzer.return_type.is_number() && first_element_type.is_number();

                all_same = all_same && same_or_compatible_types;
            }

            if !all_same {
                diagnosis.push(SemanticError::InvalidArrayElement)
            }

            return_type = LangType::Array(Box::new(first_element_type), array.expressions.len());
        } else {
            // If the array is empty then its type is array of any.
            return_type = LangType::Array(Box::new(LangType::Any), 0);
        }

        Self {
            return_type,
            diagnosis,
        }
    }
}
