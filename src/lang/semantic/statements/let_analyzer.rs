use crate::lang::semantic::expression_analyzer::ExpressionAnalyzer;
use crate::lang::semantic::scope::Scope;
use crate::lang::semantic::semantic_error::SemanticError;
use crate::lang::semantic::semantic_type::SemanticType;
use crate::lang::semantic::shared::type_analyzer::TypeAnalyzer;
use crate::lang::semantic::symbol::Symbol;
use crate::lang::syntax::parser::statements::r#let::Let;

use std::{cell::RefCell, rc::Rc};

/// Analyzer responsible for semantic analysis of 'let' statements.
pub struct LetAnalyzer {
    /// A collection of semantic errors found during analysis.
    pub(crate) diagnosis: Vec<SemanticError>,
}

impl LetAnalyzer {
    /// Analyzes the provided 'let' statement within a given scope.
    ///
    /// # Arguments
    ///
    /// * `r#let` - A reference to the 'let' statement to be analyzed.
    /// * `scope` - A reference-counted reference to the scope in which the analysis occurs.
    ///
    /// # Returns
    ///
    /// A `LetAnalyzer` instance containing the analysis results.
    pub fn analyze(r#let: &Let, scope: Rc<RefCell<Scope>>) -> Self {
        let mut diagnosis: Vec<SemanticError> = vec![];
        let mut expression_type = SemanticType::Any;
        let mut variable_type = SemanticType::Any;

        let variable_name = r#let.identifier.name.clone();

        // Verify if the function was already declared or if some builtin identifier has the same name.
        if let Some(_) = scope.borrow().get(&variable_name) {
            diagnosis.push(SemanticError::DuplicatedIdentifier);
        }

        if let Some(expression) = &r#let.expression {
            let analyzer = ExpressionAnalyzer::analyze(expression, Rc::clone(&scope));

            diagnosis.extend(analyzer.diagnosis);

            expression_type = analyzer.return_type;
            variable_type = expression_type.clone();
        }

        if let Some(r#type) = &r#let.r#type {
            let analyzer = TypeAnalyzer::analyze(r#type, Rc::clone(&scope));
            diagnosis.extend(analyzer.diagnosis);
            variable_type = analyzer.result_type;
        }

        // The code will always use the explicit type in case mismatched types. That's way the variable_type receives the result of the TypeAnalyzer analyses.

        if r#let.r#type.is_some() && r#let.expression.is_some() {
            let both_numbers = variable_type.is_number() && expression_type.is_number();

            // Verify if the numbers are different and if they both are not numbers.
            if !both_numbers && variable_type != expression_type {
                diagnosis.push(SemanticError::ExpectedType {
                    expected: variable_type.clone(),
                    found: expression_type.clone(),
                });
            }
        }

        if r#let.r#type.is_none() && r#let.expression.is_none() {
            diagnosis.push(SemanticError::MissingTypeOrExpression);
        }

        // Adds the new variable in the symbol table.
        scope.borrow_mut().insert(Symbol::Variable {
            name: variable_name.clone(),
            symbol_type: variable_type.clone(),
        });

        Self { diagnosis }
    }
}
