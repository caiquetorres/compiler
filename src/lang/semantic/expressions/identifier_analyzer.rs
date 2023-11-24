use std::cell::RefCell;
use std::rc::Rc;

use crate::lang::semantic::scope::Scope;
use crate::lang::semantic::semantic_error::SemanticError;
use crate::lang::semantic::semantic_type::SemanticType;
use crate::lang::semantic::symbol::Symbol;
use crate::lang::syntax::parser::expressions::expression::ExpressionMeta;
use crate::lang::syntax::parser::shared::identifier::Identifier;

use super::expression_meta_analyzer::ExpressionMetaAnalyzer;

pub struct IdentifierAnalyzer {
    pub changeable: bool,
    pub return_type: SemanticType,
    pub diagnosis: Vec<SemanticError>,
}

impl IdentifierAnalyzer {
    pub fn analyze(
        identifier: &Identifier,
        meta: &Option<ExpressionMeta>,
        scope: Rc<RefCell<Scope>>,
    ) -> Self {
        let changeable: bool;
        let mut return_type = SemanticType::Any;
        let mut diagnosis: Vec<SemanticError> = vec![];

        let identifier_name = identifier.name.clone();

        if let Some(symbol) = scope.borrow().get(&identifier_name) {
            match symbol {
                Symbol::Variable { symbol_type, .. }
                | Symbol::Parameter { symbol_type, .. }
                | Symbol::Function { symbol_type, .. } => {
                    if let Some(meta) = &meta {
                        let analyzer =
                            ExpressionMetaAnalyzer::analyze(&symbol_type, &meta, Rc::clone(&scope));
                        diagnosis.extend(analyzer.diagnosis);

                        changeable = analyzer.changeable;
                        return_type = analyzer.return_type;
                    } else {
                        // REVIEW: Even for Symbol::Function?
                        changeable = true;
                        return_type = symbol_type.clone();
                    }
                }
                _ => {
                    changeable = true;
                    diagnosis.push(SemanticError::IdentifierNotVariableConstOrParam);
                }
            }
        } else {
            // The variable/constant/param/function is not registered on the symbol table

            changeable = true;
            diagnosis.push(SemanticError::IdentifierNotFound);
        }

        Self {
            changeable,
            return_type,
            diagnosis,
        }
    }
}
