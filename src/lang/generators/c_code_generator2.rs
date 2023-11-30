use std::collections::BTreeSet;
use std::collections::{hash_map::DefaultHasher, HashMap};
use std::hash::{Hash, Hasher};

use crate::lang::semantic::analyzer::Scopes;
use crate::lang::semantic::semantic_type::SemanticType;
use crate::lang::syntax::compilation_unit::CompilationUnit;
use crate::lang::syntax::top_level_statements::top_level_statement::TopLevelStatement;

use super::function_generator::FunctionGenerator;

fn hash_type(r#type: &SemanticType) -> u64 {
    let mut hasher = DefaultHasher::new();
    r#type.hash(&mut hasher);
    hasher.finish()
}

impl SemanticType {
    fn to_c_type(&self) -> String {
        match &self {
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
            SemanticType::Any => "void*",
            _ => panic!("Unidentified type"),
        }
        .to_string()
    }
}

// REVIEW: Should we use a common Vec instead of a BTreeSet?

#[derive(Clone, Debug)]
pub struct CCode {
    content: String,
    imports: BTreeSet<String>,
    typedefs: Vec<String>,
    types_map: HashMap<u64, String>,
}

impl CCode {
    pub fn new() -> Self {
        Self {
            content: String::new(),
            imports: BTreeSet::new(),
            typedefs: Vec::new(),
            types_map: HashMap::new(),
        }
    }

    pub fn content(&self) -> String {
        let imports: String = self
            .imports
            .iter()
            .map(|value| value.to_string())
            .collect::<Vec<String>>()
            .join("\n");

        let typedefs: String = self
            .typedefs
            .iter()
            .map(|value| value.to_string())
            .collect::<Vec<String>>()
            .join("");

        format!("{}\n{}{}", imports, typedefs, self.content)
    }

    pub fn pop(&mut self) {
        self.content.pop();
    }

    pub fn push(&mut self, code: &str) {
        self.content.push_str(code);
    }

    pub fn push_import(&mut self, import: &str) {
        self.imports.insert(String::from(import));
    }

    pub fn get_type(&mut self, r#type: SemanticType) -> String {
        let hash = hash_type(&r#type);

        match &r#type {
            SemanticType::String => {
                let value = String::from("typedef char __string[256];");

                if !self.typedefs.contains(&value) {
                    self.typedefs.push(value);
                }

                self.types_map.insert(hash, "__string".to_string());
            }
            SemanticType::Function(params, return_type) => {
                let c_return_type = self.get_type(return_type.as_ref().clone());

                let c_params: Vec<String> = params
                    .iter()
                    .map(|param| self.get_type(param.clone()))
                    .collect();

                let alias = format!("__fn_{hash}");

                let value = format!(
                    "typedef {} (*{})({});",
                    c_return_type,
                    alias,
                    c_params.join(",")
                );

                if !self.typedefs.contains(&value) {
                    self.typedefs.push(value);
                }

                self.types_map.insert(hash, alias);
            }
            SemanticType::Array(array_type, _) => {
                let c_root_type = self.get_type(get_array_root_type(&array_type));

                let dimensions = get_next_array_dimensions(&array_type)
                    .iter()
                    .map(|d| format!("[{}]", d))
                    .collect::<Vec<String>>()
                    .join("");

                let alias = format!("__array_{hash}");

                let value = format!("typedef {} (*{}){};", c_root_type, alias, dimensions);

                if !self.typedefs.contains(&value) {
                    self.typedefs.push(value);
                }

                self.types_map.insert(hash, alias);
            }
            _ => {
                self.types_map.insert(hash, r#type.to_c_type());
            }
        }

        self.types_map.get(&hash).unwrap().clone()
    }
}

pub struct CCodeGenerator2<'s, 'a> {
    scopes: &'s Scopes,
    ast: &'a CompilationUnit,
}

impl<'s, 'a> CCodeGenerator2<'s, 'a> {
    pub fn new(ast: &'a CompilationUnit, scopes: &'s Scopes) -> Self {
        Self { ast, scopes }
    }

    pub fn generate(&mut self) -> CCode {
        let mut ccode = CCode::new();

        for statement in &self.ast.statements {
            match statement {
                TopLevelStatement::Function(function) => {
                    FunctionGenerator::generate_prototype(function, &mut ccode)
                }
            }
        }

        for statement in &self.ast.statements {
            match statement {
                TopLevelStatement::Function(function) => {
                    FunctionGenerator::generate(function, self.scopes, &mut ccode)
                }
            }
        }

        ccode
    }
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
