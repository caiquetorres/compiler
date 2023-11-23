use std::cell::RefCell;
use std::rc::Rc;

use crate::lang::semantic::expression_analyzer::ExpressionAnalyzer;
use crate::lang::semantic::scope::Scope;
use crate::lang::semantic::semantic_error::SemanticError;
use crate::lang::semantic::semantic_type::SemanticType;
use crate::lang::syntax::parser::expressions::parenthesized::Parenthesized;

pub struct ParenthesizedAnalyzer {
    pub return_type: SemanticType,
    pub diagnosis: Vec<SemanticError>,
}

impl ParenthesizedAnalyzer {
    pub fn analyze(parenthesized: &Parenthesized, scope: Rc<RefCell<Scope>>) -> Self {
        let mut diagnosis: Vec<SemanticError> = vec![];

        let analyzer = ExpressionAnalyzer::analyze(&parenthesized.expression, Rc::clone(&scope));

        diagnosis.extend(analyzer.diagnosis);

        let return_type = analyzer.return_type;

        Self {
            return_type,
            diagnosis,
        }
    }
}
