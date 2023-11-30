use std::rc::Rc;

use crate::lang::{
    lexer::token_kind::TokenKind,
    semantic::{analyzer::Scopes, symbol::Symbol},
    syntax::{expressions::expression::Expression, statements::r#for::For},
};

use super::{
    block_generator::BlockGenerator, c_code_generator2::CCode,
    expression_generator::ExpressionGenerator,
};

pub struct ForGenerator;

impl ForGenerator {
    pub fn generate(r#for: &For, scopes: &Scopes, ccode: &mut CCode) {
        let scope = scopes.get(&r#for.block.id).unwrap().clone();

        if let Expression::Range(range) = &r#for.expression {
            let symbol = scope.borrow().get(&r#for.identifier.name).unwrap();

            ccode.push("for(");

            if let Symbol::Variable { symbol_type, .. } = &symbol {
                let c_type = ccode.get_type(symbol_type.clone());
                ccode.push(&format!("{} {}=", c_type, r#for.identifier.name))
            }

            let code = ExpressionGenerator::generate(&range.left, Rc::clone(&scope), ccode);
            ccode.push(&code);

            ccode.push(";");

            match range.operator.token.kind {
                TokenKind::DotDot => ccode.push(&format!("{}<", r#for.identifier.name)),
                TokenKind::DotDotEquals => ccode.push(&format!("{}<=", r#for.identifier.name)),
                _ => unreachable!(),
            }

            let code = ExpressionGenerator::generate(&range.right, Rc::clone(&scope), ccode);
            ccode.push(&code);

            ccode.push(";");
            ccode.push(&format!("{}++)", r#for.identifier.name));
            BlockGenerator::generate(&r#for.block, scopes, ccode);
        }
    }
}
