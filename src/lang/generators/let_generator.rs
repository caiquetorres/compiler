use std::{cell::RefCell, rc::Rc};

use crate::lang::{
    semantic::{scope::Scope, semantic_type::SemanticType, symbol::Symbol},
    syntax::statements::r#let::Let,
};

use super::{
    array_generator::ArrayGenerator, c_code_generator2::CCode,
    expression_generator::ExpressionGenerator,
};

pub struct LetGenerator;

impl LetGenerator {
    pub fn generate(r#let: &Let, scope: Rc<RefCell<Scope>>, ccode: &mut CCode) {
        let identifier_name = r#let.identifier.name.clone();
        let type_identifier = scope.borrow().get(&identifier_name).unwrap();

        if let Symbol::Variable { symbol_type, .. } = type_identifier {
            let c_type = ccode.get_type(symbol_type.clone());
            ccode.push(&format!("{} {}", c_type, identifier_name.clone()));

            if let Some(expression) = &r#let.expression {
                ccode.push("=");
                let code = ExpressionGenerator::generate(expression, Rc::clone(&scope), ccode);
                ccode.push(&code);
            } else {
                if let SemanticType::Array(_, _) = symbol_type {
                    ccode.push("=");
                    let value = &ArrayGenerator::generate_expression(
                        &symbol_type,
                        &vec![],
                        &None,
                        Rc::clone(&scope),
                        ccode,
                    );
                    ccode.push(value);
                }
            }
        }

        ccode.push(";")
    }
}
