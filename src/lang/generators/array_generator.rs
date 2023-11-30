use std::{cell::RefCell, rc::Rc};

use crate::lang::{
    generators::expression_meta_generator::ExpressionMetaGenerator,
    semantic::{scope::Scope, semantic_type::SemanticType},
    syntax::expressions::expression::{Expression, ExpressionMeta},
};

use super::{c_code_generator2::CCode, expression_generator::ExpressionGenerator};

pub struct ArrayGenerator;

impl ArrayGenerator {
    pub fn generate_expression(
        array_type: &SemanticType,
        expressions: &Vec<Expression>,
        meta: &Option<ExpressionMeta>,
        scope: Rc<RefCell<Scope>>,
        ccode: &mut CCode,
    ) -> String {
        let root_type = Self::get_array_root_type(&array_type);

        let dimensions = Self::get_next_array_dimensions(&array_type);

        let expressions = Self::generate_expressions(expressions, Rc::clone(&scope), ccode);

        if let Some(meta) = meta {
            format!(
                "({}{}){{{}}}{}",
                ccode.get_type(root_type),
                dimensions
                    .iter()
                    .map(|d| format!("[{}]", d))
                    .collect::<Vec<String>>()
                    .join(""),
                expressions.join(","),
                ExpressionMetaGenerator::generate(meta, Rc::clone(&scope), ccode),
            )
        } else {
            format!(
                "({}{}){{{}}}",
                ccode.get_type(root_type),
                dimensions
                    .iter()
                    .map(|d| format!("[{}]", d))
                    .collect::<Vec<String>>()
                    .join(""),
                expressions.join(",")
            )
        }
    }

    fn generate_expressions(
        expressions: &Vec<Expression>,
        scope: Rc<RefCell<Scope>>,
        ccode: &mut CCode,
    ) -> Vec<String> {
        let mut results: Vec<String> = vec![];

        for expression in expressions {
            if let Expression::Array(array, ..) = expression {
                results.extend(Self::generate_expressions(
                    &array.expressions,
                    Rc::clone(&scope),
                    ccode,
                ));
            } else {
                results.push(ExpressionGenerator::generate(
                    expression,
                    Rc::clone(&scope),
                    ccode,
                ));
            }
        }

        results
    }

    fn get_next_array_dimensions(r#type: &SemanticType) -> Vec<usize> {
        let mut dimensions: Vec<usize> = vec![];
        let mut current_type = r#type.clone();

        loop {
            if let SemanticType::Array(array_type, size) = current_type {
                current_type = array_type.as_ref().clone();
                dimensions.push(size);
            } else {
                break;
            }
        }

        dimensions
    }

    fn get_array_root_type(r#type: &SemanticType) -> SemanticType {
        let mut root_type = r#type.clone();

        loop {
            if let SemanticType::Array(array_type, _) = &root_type {
                root_type = array_type.as_ref().clone();
            } else {
                break;
            }
        }

        return root_type;
    }
}
