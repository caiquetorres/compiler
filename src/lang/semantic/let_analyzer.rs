use std::{cell::RefCell, rc::Rc};

use crate::lang::syntax::parser::statements::r#let::Let;

use super::{
    expression_analyzer::ExpressionAnalyzer, lang_type::LangType, scope::Scope,
    semantic_error::SemanticError, symbol::Symbol,
};

pub struct LetAnalyzer {
    pub(crate) diagnosis: Vec<SemanticError>,
}

impl LetAnalyzer {
    pub fn analyze(r#let: &Let, scope: Rc<RefCell<Scope>>) -> Self {
        let mut diagnosis: Vec<SemanticError> = vec![];

        let variable_name = r#let.identifier.name.clone();

        let mut expression_type = LangType::Any;
        let mut variable_type = LangType::Any;

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

        if let Some(type_identifier) = &r#let.type_identifier {
            let variable_type_name = type_identifier.name.clone();

            // Verify if the function return type exists.
            if let None = scope.borrow().get(&variable_type_name) {
                diagnosis.push(SemanticError::IdentifierNotFound);
            }

            variable_type = LangType::from(variable_type_name);
        }

        if r#let.type_identifier.is_some() && r#let.expression.is_some() {
            if !variable_type.is_number() && variable_type != expression_type {
                diagnosis.push(SemanticError::ExpectedType {
                    expected: variable_type.clone(),
                    found: expression_type.clone(),
                });
            }
        }

        if r#let.type_identifier.is_none() && r#let.expression.is_none() {
            diagnosis.push(SemanticError::MissingTypeOrExpression);
        }

        scope.borrow_mut().insert(Symbol::Variable {
            name: variable_name.clone(),
            symbol_type: variable_type.clone(),
        });

        Self { diagnosis }
    }
}
