use std::cell::RefCell;
use std::rc::Rc;

use crate::lang::semantic::scope::Scope;
use crate::lang::semantic::semantic_error::SemanticError;
use crate::lang::semantic::semantic_type::SemanticType;
use crate::lang::syntax::parser::expressions::expression::ExpressionMeta;
use crate::lang::syntax::parser::expressions::parenthesized::Parenthesized;

use super::expression_analyzer::{ExpressionAnalyzer, ExpressionMetaAnalyzer};

pub struct ParenthesizedAnalyzer {
    pub return_type: SemanticType,
    pub diagnosis: Vec<SemanticError>,
}

impl ParenthesizedAnalyzer {
    pub fn analyze(
        parenthesized: &Parenthesized,
        meta: &Option<ExpressionMeta>,
        scope: Rc<RefCell<Scope>>,
    ) -> Self {
        let mut diagnosis: Vec<SemanticError> = vec![];

        let analyzer = ExpressionAnalyzer::analyze(&parenthesized.expression, Rc::clone(&scope));

        diagnosis.extend(analyzer.diagnosis);

        let return_type: SemanticType;

        if let Some(meta) = &meta {
            let analyzer =
                ExpressionMetaAnalyzer::analyze(&analyzer.return_type, &meta, Rc::clone(&scope));
            diagnosis.extend(analyzer.diagnosis);
            return_type = analyzer.return_type;
        } else {
            return_type = analyzer.return_type;
        }

        Self {
            return_type,
            diagnosis,
        }
    }
}
