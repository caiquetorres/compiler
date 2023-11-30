use std::{cell::RefCell, rc::Rc};

use crate::lang::{
    semantic::{
        expressions::expression_analyzer::ExpressionAnalyzer, scope::Scope,
        semantic_type::SemanticType,
    },
    syntax::statements::print::Print,
};

use super::{c_code_generator2::CCode, expression_generator::ExpressionGenerator};

pub struct PrintGenerator;

impl PrintGenerator {
    pub fn generate(print: &Print, scope: Rc<RefCell<Scope>>, ccode: &mut CCode) {
        ccode.push_import("#include<stdio.h>");

        for expression in &print.expressions {
            ccode.push("printf(\"");

            let analyzer = ExpressionAnalyzer::analyze(expression, Rc::clone(&scope));
            let return_type = analyzer.return_type;

            let c_print_shortcut = match &return_type {
                SemanticType::String => "%s",
                SemanticType::I8 => "%d",
                SemanticType::U8 => "%u",
                SemanticType::I16 => "%d",
                SemanticType::U16 => "%u",
                SemanticType::I32 => "%d",
                SemanticType::U32 => "%u",
                SemanticType::I64 => "%lld",
                SemanticType::U64 => "%llu",
                SemanticType::F32 => "%ff",
                SemanticType::F64 => "%lf",
                SemanticType::Bool => "%s",
                SemanticType::Char => "%c",
                SemanticType::Ref(_) => "%p",
                _ => "",
            };

            ccode.push(&format!("{}", c_print_shortcut));
            ccode.push("\",");

            let value = ExpressionGenerator::generate(expression, Rc::clone(&scope), ccode);
            ccode.push(&value);

            if return_type.is_bool() {
                ccode.push("?\"true\":\"false\"");
            }

            ccode.push(");");
        }

        if print.new_line {
            ccode.push("printf(\"\\n\");");
        }
    }
}
