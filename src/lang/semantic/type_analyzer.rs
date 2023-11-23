use std::{cell::RefCell, rc::Rc};

use crate::lang::syntax::{lexer::token_kind::TokenKind, parser::shared::r#type::Type};

use super::{lang_type::LangType, scope::Scope, semantic_error::SemanticError};

pub struct TypeAnalyzer {
    pub(crate) result_type: LangType,
    pub(crate) diagnosis: Vec<SemanticError>,
}

impl TypeAnalyzer {
    pub fn analyze(r#type: &Type, scope: Rc<RefCell<Scope>>) -> Self {
        let mut lang_type = LangType::Any;
        let mut diagnosis: Vec<SemanticError> = vec![];

        match r#type {
            Type::Simple { identifier } => {
                let variable_type_name = identifier.name.clone();

                if let None = scope.borrow().get(&variable_type_name) {
                    diagnosis.push(SemanticError::IdentifierNotFound);
                }

                lang_type = LangType::from(variable_type_name);
            }
            Type::Array { r#type, size } => {
                let analyzer = TypeAnalyzer::analyze(r#type, Rc::clone(&scope));

                diagnosis.extend(analyzer.diagnosis);

                if let TokenKind::NumberLiteral = size.kind {
                    let number = size.value.parse::<usize>();

                    if let Ok(number) = number {
                        lang_type =
                            LangType::Array(Box::new(LangType::from(analyzer.result_type)), number);
                    }
                }
            }
            Type::Reference { inner_type } => {
                let analyzer = Self::analyze(inner_type, Rc::clone(&scope));

                diagnosis.extend(analyzer.diagnosis);

                lang_type = LangType::Ref(Box::new(analyzer.result_type));
            }
        }

        Self {
            result_type: lang_type,
            diagnosis,
        }
    }
}
