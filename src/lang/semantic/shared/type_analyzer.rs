use crate::lang::lexer::token::Token;
use crate::lang::lexer::token_kind::TokenKind;
use crate::lang::semantic::{
    scope::Scope, semantic_error::SemanticError, semantic_type::SemanticType,
};
use crate::lang::syntax::parser::shared::syntax_type::SyntaxType;

use std::{cell::RefCell, rc::Rc};

/// Analyzer responsible for performing semantic analysis of types.
pub struct TypeAnalyzer {
    /// The inferred result type after semantic analysis.
    pub(crate) result_type: SemanticType,

    /// A collection of semantic errors found during analysis.
    pub(crate) diagnosis: Vec<SemanticError>,
}

impl TypeAnalyzer {
    /// Analyzes the provided type within a given scope.
    ///
    /// # Arguments
    ///
    /// * `r#type` - A reference to the type to be analyzed.
    /// * `scope` - A reference-counted reference to the scope in which the analysis occurs.
    ///
    /// # Returns
    ///
    /// A `TypeAnalyzer` instance containing the analysis results.
    pub fn analyze(r#type: &SyntaxType, scope: Rc<RefCell<Scope>>) -> Self {
        match r#type {
            SyntaxType::Simple { identifier } => {
                Self::analyze_simple_type(identifier, Rc::clone(&scope))
            }
            SyntaxType::Array { r#type, size } => {
                Self::analyze_array_type(r#type, size, Rc::clone(&scope))
            }
            SyntaxType::Reference { inner_type } => {
                Self::analyze_reference_type(inner_type, Rc::clone(&scope))
            }
        }
    }

    /// Analyzes a simple type based on an identifier token within a given scope.
    ///
    /// # Arguments
    ///
    /// * `token_identifier` - A reference to the identifier token representing the simple type.
    /// * `scope` - A reference-counted reference to the scope in which the analysis occurs.
    ///
    /// # Returns
    ///
    /// A `TypeAnalyzer` instance containing the analysis results.
    fn analyze_simple_type(token_identifier: &Token, scope: Rc<RefCell<Scope>>) -> Self {
        let mut diagnosis: Vec<SemanticError> = vec![];
        let mut result_type = SemanticType::Any;

        let variable_type_name = token_identifier.value.clone();

        if let Some(_) = scope.borrow().get(&variable_type_name) {
            result_type = SemanticType::from(variable_type_name);
        } else {
            diagnosis.push(SemanticError::IdentifierNotFound);
        }

        Self {
            result_type,
            diagnosis,
        }
    }

    /// Analyzes an array type based on the type and size within a given scope.
    ///
    /// # Arguments
    ///
    /// * `r#type` - A reference to the type contained within the array.
    /// * `size` - A reference to the token representing the size of the array.
    /// * `scope` - A reference-counted reference to the scope in which the analysis occurs.
    ///
    /// # Returns
    ///
    /// A `TypeAnalyzer` instance containing the analysis results.
    fn analyze_array_type(
        r#type: &Box<SyntaxType>,
        size: &Token,
        scope: Rc<RefCell<Scope>>,
    ) -> Self {
        let mut result_type = SemanticType::Any;
        let mut diagnosis: Vec<SemanticError> = vec![];

        let analyzer = TypeAnalyzer::analyze(r#type, Rc::clone(&scope));

        diagnosis.extend(analyzer.diagnosis);

        if let TokenKind::NumberLiteral = size.kind {
            let number = size.value.parse::<usize>();

            // TODO: Check if the literal type is integer.

            if let Ok(number) = number {
                result_type =
                    SemanticType::Array(Box::new(SemanticType::from(analyzer.result_type)), number);
            }
        }

        Self {
            result_type,
            diagnosis,
        }
    }

    /// Analyzes a reference type based on the inner type within a given scope.
    ///
    /// # Arguments
    ///
    /// * `inner_type` - A reference to the type that the reference is pointing to.
    /// * `scope` - A reference-counted reference to the scope in which the analysis occurs.
    ///
    /// # Returns
    ///
    /// A `TypeAnalyzer` instance containing the analysis results.
    fn analyze_reference_type(inner_type: &Box<SyntaxType>, scope: Rc<RefCell<Scope>>) -> Self {
        let mut diagnosis: Vec<SemanticError> = vec![];
        let analyzer = Self::analyze(inner_type, Rc::clone(&scope));

        diagnosis.extend(analyzer.diagnosis);

        let result_type = SemanticType::Ref(Box::new(analyzer.result_type));

        Self {
            result_type,
            diagnosis,
        }
    }
}
