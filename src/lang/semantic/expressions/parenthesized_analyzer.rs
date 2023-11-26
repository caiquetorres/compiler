use std::cell::RefCell;
use std::rc::Rc;

use crate::lang::position::Positioned;
use crate::lang::semantic::scope::Scope;
use crate::lang::semantic::semantic_error::SemanticError;
use crate::lang::semantic::semantic_type::SemanticType;
use crate::lang::syntax::expressions::expression::{Expression, ExpressionMeta};
use crate::lang::syntax::expressions::parenthesized::Parenthesized;

use super::expression_analyzer::ExpressionAnalyzer;
use super::expression_meta_analyzer::ExpressionMetaAnalyzer;

pub struct ParenthesizedAnalyzer {
    pub changeable: bool,
    pub return_type: SemanticType,
    pub diagnosis: Vec<SemanticError>,
}

impl ParenthesizedAnalyzer {
    pub fn analyze(
        parenthesized: &Parenthesized,
        meta: &Option<ExpressionMeta>,
        scope: Rc<RefCell<Scope>>,
    ) -> Self {
        let changeable: bool;
        let return_type: SemanticType;
        let mut diagnosis: Vec<SemanticError> = vec![];

        let analyzer = ExpressionAnalyzer::analyze(&parenthesized.expression, Rc::clone(&scope));

        diagnosis.extend(analyzer.diagnosis);

        if let Expression::Array(_) = parenthesized.expression.as_ref() {
            diagnosis.push(SemanticError::ImmediateArrayUsageWithoutAssignment {
                position: parenthesized.expression.get_position(),
            });
            return_type = SemanticType::Any;
            changeable = analyzer.changeable;
        } else {
            if let Some(meta) = &meta {
                let analyzer = ExpressionMetaAnalyzer::analyze(
                    &analyzer.return_type,
                    &meta,
                    Rc::clone(&scope),
                );
                diagnosis.extend(analyzer.diagnosis);

                changeable = analyzer.changeable;
                return_type = analyzer.return_type;
            } else {
                changeable = analyzer.changeable;
                return_type = analyzer.return_type;
            }
        }

        Self {
            changeable,
            return_type,
            diagnosis,
        }
    }
}
