use std::{cell::RefCell, rc::Rc};

use crate::lang::syntax::parser::{expressions::expression::Expression, statements::r#for::For};

use super::{
    analyzer::Scopes, block_analyzer::BlockAnalyzer, expression_analyzer::ExpressionAnalyzer,
    lang_type::LangType, scope::Scope, semantic_error::SemanticError, symbol::Symbol,
};

pub struct ForAnalyzer {
    pub(crate) diagnosis: Vec<SemanticError>,
}

impl ForAnalyzer {
    pub fn analyze(r#for: &For, scope: Rc<RefCell<Scope>>, scopes: &mut Scopes) -> Self {
        let mut diagnosis: Vec<SemanticError> = vec![];

        let scope = Rc::new(RefCell::new(Scope::new(Rc::clone(&scope), true, None)));

        let identifier_name = r#for.identifier.name.clone();

        if let Some(_) = scope.borrow().get(&identifier_name) {
            diagnosis.push(SemanticError::DuplicatedIdentifier);
        }

        scope.borrow_mut().insert(Symbol::Const {
            name: identifier_name,
            symbol_type: LangType::I32,
        });

        let analyzer = ExpressionAnalyzer::analyze(&r#for.expression, Rc::clone(&scope));

        diagnosis.extend(analyzer.diagnosis);

        if let Expression::Range(_) = &r#for.expression {
            let analyzer =
                BlockAnalyzer::analyze_within_scope(&r#for.block, Rc::clone(&scope), scopes);

            diagnosis.extend(analyzer.diagnosis);
        } else {
            diagnosis.push(SemanticError::ExpectedType {
                expected: LangType::Range,
                found: analyzer.return_type,
            });
        }

        Self { diagnosis }
    }
}
