use std::{cell::RefCell, rc::Rc};

use crate::lang::{
    generators::expression_meta_generator::ExpressionMetaGenerator,
    semantic::{scope::Scope, semantic_type::SemanticType},
    syntax::expressions::expression::{Expression, ExpressionMeta},
};

use super::expression_generator::ExpressionGenerator;

fn convert_type(r#type: SemanticType) -> String {
    match r#type {
        SemanticType::Void => "void",
        SemanticType::Char => "unsigned char",
        SemanticType::Bool => "unsigned char",
        SemanticType::U8 => "unsigned char",
        SemanticType::I8 => "signed char",
        SemanticType::U16 => "unsigned short int",
        SemanticType::I16 => "signed short int",
        SemanticType::U32 => "unsigned int",
        SemanticType::I32 => "signed int",
        SemanticType::U64 => "unsigned long long int",
        SemanticType::I64 => "signed long long int",
        SemanticType::F32 => "float",
        SemanticType::F64 => "double",
        SemanticType::String => "char*",
        SemanticType::Any => "void *",
        _ => panic!("Something wrong is not right"),
    }
    .to_string()
}

pub struct ArrayGenerator;

impl ArrayGenerator {
    pub fn generate_declaration(
        identifier_name: Option<String>,
        array_type: &SemanticType,
    ) -> String {
        let root_type = Self::get_array_root_type(&array_type);

        let dimensions = Self::get_next_array_dimensions(&array_type);

        if let Some(identifier_name) = identifier_name {
            format!(
                "{}(*{}){}",
                convert_type(root_type),
                identifier_name,
                dimensions
                    .iter()
                    .skip(1)
                    .map(|d| format!("[{}]", d))
                    .collect::<Vec<String>>()
                    .join("")
            )
        } else {
            format!(
                "{}(*){}",
                convert_type(root_type),
                dimensions
                    .iter()
                    .skip(1)
                    .map(|d| format!("[{}]", d))
                    .collect::<Vec<String>>()
                    .join("")
            )
        }
    }

    pub fn generate_expression(
        array_type: &SemanticType,
        expressions: &Vec<Expression>,
        meta: &Option<ExpressionMeta>,
        scope: Rc<RefCell<Scope>>,
    ) -> String {
        let root_type = Self::get_array_root_type(&array_type);

        let dimensions = Self::get_next_array_dimensions(&array_type);

        let expressions = Self::generate_expressions(expressions, Rc::clone(&scope));

        if let Some(meta) = meta {
            format!(
                "({}{}){{{}}}{}",
                convert_type(root_type),
                dimensions
                    .iter()
                    .map(|d| format!("[{}]", d))
                    .collect::<Vec<String>>()
                    .join(""),
                expressions.join(","),
                ExpressionMetaGenerator::generate(meta, Rc::clone(&scope)),
            )
        } else {
            format!(
                "({}{}){{{}}}",
                convert_type(root_type),
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
    ) -> Vec<String> {
        let mut results: Vec<String> = vec![];

        for expression in expressions {
            if let Expression::Array(array, ..) = expression {
                results.extend(Self::generate_expressions(
                    &array.expressions,
                    Rc::clone(&scope),
                ));
            } else {
                results.push(ExpressionGenerator::generate(expression, Rc::clone(&scope)));
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
