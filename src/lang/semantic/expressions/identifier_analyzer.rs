use std::cell::RefCell;
use std::rc::Rc;

use crate::lang::semantic::scope::Scope;
use crate::lang::semantic::semantic_error::SemanticError;
use crate::lang::semantic::semantic_type::SemanticType;
use crate::lang::semantic::symbol::Symbol;
use crate::lang::syntax::parser::expressions::expression::ExpressionMeta;
use crate::lang::syntax::parser::shared::identifier::Identifier;

use super::expression_analyzer::ExpressionMetaAnalyzer;

pub struct IdentifierAnalyzer {
    pub return_type: SemanticType,
    pub diagnosis: Vec<SemanticError>,
}

impl IdentifierAnalyzer {
    pub fn analyze(
        identifier: &Identifier,
        meta: &Option<ExpressionMeta>,
        scope: Rc<RefCell<Scope>>,
    ) -> Self {
        let mut return_type = SemanticType::Any;
        let mut diagnosis: Vec<SemanticError> = vec![];

        let identifier_name = identifier.name.clone();

        if let Some(symbol) = scope.borrow().get(&identifier_name) {
            match symbol {
                Symbol::Variable { symbol_type, .. }
                | Symbol::Const { symbol_type, .. }
                | Symbol::Parameter { symbol_type, .. }
                | Symbol::Function { symbol_type, .. } => {
                    if let Some(meta) = &meta {
                        let analyzer =
                            ExpressionMetaAnalyzer::analyze(&symbol_type, &meta, Rc::clone(&scope));
                        diagnosis.extend(analyzer.diagnosis);
                        return_type = analyzer.return_type;
                    } else {
                        return_type = symbol_type.clone();
                    }
                }
                _ => {
                    diagnosis.push(SemanticError::IdentifierNotVariableConstOrParam);
                }
            }
        } else {
            // The variable/constant/param/function is not registered on the symbol table
            diagnosis.push(SemanticError::IdentifierNotFound);
        }

        Self {
            return_type,
            diagnosis,
        }
    }
}
