use std::{cell::RefCell, rc::Rc};

use crate::lang::syntax::parser::statements::r#const::Const;

use super::{
    expression_analyzer::ExpressionAnalyzer, lang_type::LangType, scope::Scope,
    semantic_error::SemanticError, symbol::Symbol,
};

pub struct ConstAnalyzer {
    pub(crate) diagnosis: Vec<SemanticError>,
}

impl ConstAnalyzer {
    pub fn analyze(r#const: &Const, scope: Rc<RefCell<Scope>>) -> Self {
        let mut diagnosis: Vec<SemanticError> = vec![];

        let constant_name = r#const.identifier.name.clone();
        let mut constant_type: LangType;

        // Verify if the function was already declared or if some builtin identifier has the same name.
        if let Some(_) = scope.borrow().get(&constant_name) {
            diagnosis.push(SemanticError::DuplicatedIdentifier);
        }

        let analyzer = ExpressionAnalyzer::analyze(&r#const.expression, Rc::clone(&scope));

        diagnosis.extend(analyzer.diagnosis);

        constant_type = analyzer.return_type;

        if let Some(type_identifier) = &r#const.type_identifier {
            let constant_type_name = type_identifier.name.clone();

            // Verify if the function return type exists.
            if let None = scope.borrow().get(&constant_type_name) {
                diagnosis.push(SemanticError::IdentifierNotFound);
            }

            constant_type = LangType::from(constant_type_name);
        }

        scope.borrow_mut().insert(Symbol::Const {
            name: constant_name,
            symbol_type: constant_type,
        });

        Self { diagnosis }
    }
}
