use crate::lang::semantic::semantic_error::SemanticError;
use crate::lang::semantic::semantic_type::SemanticType;
use crate::lang::syntax::parser::expressions::array::Array;
use crate::lang::{
    semantic::scope::Scope, syntax::parser::expressions::expression::ExpressionMeta,
};

use std::{cell::RefCell, rc::Rc};

use super::expression_analyzer::ExpressionAnalyzer;
use super::expression_meta_analyzer::ExpressionMetaAnalyzer;

/// Analyzer that performs the semantic analysis for arrays.
pub struct ArrayAnalyzer {
    pub changeable: bool,

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
    pub fn analyze(
        array: &Array,
        meta: &Option<ExpressionMeta>,
        scope: Rc<RefCell<Scope>>,
    ) -> Self {
        let changeable: bool;
        let mut return_type: SemanticType;
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

            return_type =
                SemanticType::Array(Box::new(first_element_type), array.expressions.len());
        } else {
            // If the array is empty then its type is array of any.
            return_type = SemanticType::Array(Box::new(SemanticType::Any), 0);
        }

        if let Some(meta) = &meta {
            let analyzer = ExpressionMetaAnalyzer::analyze(&return_type, &meta, Rc::clone(&scope));
            diagnosis.extend(analyzer.diagnosis);

            changeable = analyzer.changeable;
            return_type = analyzer.return_type;
        } else {
            changeable = true;
        }

        Self {
            changeable,
            return_type,
            diagnosis,
        }
    }
}
