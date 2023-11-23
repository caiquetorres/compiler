use std::{cell::RefCell, rc::Rc};

use crate::lang::semantic::expression_analyzer::ExpressionAnalyzer;
use crate::lang::semantic::scope::Scope;
use crate::lang::semantic::semantic_error::SemanticError;
use crate::lang::semantic::semantic_type::SemanticType;
use crate::lang::semantic::shared::type_analyzer::TypeAnalyzer;
use crate::lang::semantic::symbol::Symbol;
use crate::lang::syntax::parser::statements::r#const::Const;

/// Analyzer responsible for semantic analysis of constants.
pub struct ConstAnalyzer {
    /// A collection of semantic errors found during analysis.
    pub(crate) diagnosis: Vec<SemanticError>,
}

impl ConstAnalyzer {
    /// Analyzes the provided constant within a given scope.
    ///
    /// # Arguments
    ///
    /// * `r#const` - A reference to the constant to be analyzed.
    /// * `scope` - A reference-counted reference to the scope in which the analysis occurs.
    ///
    /// # Returns
    ///
    /// A `ConstAnalyzer` instance containing the analysis results.
    pub fn analyze(r#const: &Const, scope: Rc<RefCell<Scope>>) -> Self {
        let mut diagnosis: Vec<SemanticError> = vec![];

        let constant_name = r#const.identifier.name.clone();
        let mut constant_type: SemanticType;

        // Verify if the function was already declared or if some builtin identifier has the same name.
        if let Some(_) = scope.borrow().get(&constant_name) {
            diagnosis.push(SemanticError::DuplicatedIdentifier);
        }

        let analyzer = ExpressionAnalyzer::analyze(&r#const.expression, Rc::clone(&scope));

        diagnosis.extend(analyzer.diagnosis);

        constant_type = analyzer.return_type;

        if let Some(r#type) = &r#const.r#type {
            let analyzer = TypeAnalyzer::analyze(r#type, Rc::clone(&scope));
            diagnosis.extend(analyzer.diagnosis);
            constant_type = analyzer.result_type;
        }

        // Adds the new constant in the symbol table.
        scope.borrow_mut().insert(Symbol::Const {
            name: constant_name,
            symbol_type: constant_type,
        });

        Self { diagnosis }
    }
}
