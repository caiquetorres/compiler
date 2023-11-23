use std::fmt;

use crate::lang::syntax::parser::shared::r#type::Type;

#[derive(Clone, Debug, PartialEq)]
pub enum SemanticType {
    U8,
    I8,
    U16,
    I16,
    U32,
    I32,
    U64,
    I64,
    F32,
    F64,
    Void,
    Bool,
    Char,
    String,
    Range,
    Any,
    // Custom(String),
    Ref(Box<SemanticType>),
    Array(Box<SemanticType>, usize),
}

impl SemanticType {
    pub fn number_type_precedence(v: Vec<SemanticType>) -> SemanticType {
        if v.contains(&SemanticType::Any) {
            SemanticType::Any
        } else if v.contains(&SemanticType::F64) {
            SemanticType::F64
        } else if v.contains(&SemanticType::F32) {
            SemanticType::F32
        } else if v.contains(&SemanticType::I64) {
            SemanticType::I64
        } else if v.contains(&SemanticType::U64) {
            SemanticType::U64
        } else if v.contains(&SemanticType::I32) {
            SemanticType::I32
        } else if v.contains(&SemanticType::U32) {
            SemanticType::U32
        } else if v.contains(&SemanticType::I16) {
            SemanticType::I16
        } else if v.contains(&SemanticType::U16) {
            SemanticType::U16
        } else if v.contains(&SemanticType::I8) {
            SemanticType::I8
        } else {
            SemanticType::U8
        }
    }

    pub fn is_integer(&self) -> bool {
        matches!(
            &self,
            Self::Any
                | Self::U8
                | Self::I8
                | Self::U16
                | Self::I16
                | Self::U32
                | Self::I32
                | Self::U64
                | Self::I64
        )
    }

    pub fn is_bool(&self) -> bool {
        matches!(&self, Self::Any | Self::Bool)
    }

    pub fn is_number(&self) -> bool {
        matches!(
            &self,
            Self::Any
                | Self::U8
                | Self::I8
                | Self::U16
                | Self::I16
                | Self::U32
                | Self::I32
                | Self::U64
                | Self::I64
                | Self::F32
                | Self::F64
        )
    }

    pub fn from_type(r#type: Type) -> Self {
        match r#type {
            Type::Simple { identifier } => Self::from(identifier.value),
            Type::Array { r#type, size } => {
                let size = size.value.parse::<usize>().unwrap();
                Self::Array(Box::new(Self::from_type(r#type.as_ref().clone())), size)
            }
            Type::Reference { inner_type } => {
                Self::Ref(Box::new(Self::from_type(inner_type.as_ref().clone())))
            }
        }
    }
}

impl fmt::Display for SemanticType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let value = match self {
            SemanticType::Void => "void".to_string(),
            SemanticType::I8 => "i8".to_string(),
            SemanticType::I16 => "i16".to_string(),
            SemanticType::I32 => "i32".to_string(),
            SemanticType::I64 => "i64".to_string(),
            SemanticType::U8 => "u8".to_string(),
            SemanticType::U16 => "u16".to_string(),
            SemanticType::U32 => "u32".to_string(),
            SemanticType::U64 => "u64".to_string(),
            SemanticType::F32 => "f32".to_string(),
            SemanticType::F64 => "f64".to_string(),
            SemanticType::Bool => "bool".to_string(),
            SemanticType::Char => "char".to_string(),
            SemanticType::String => "string".to_string(),
            SemanticType::Range => "range".to_string(),
            SemanticType::Any => "any".to_string(),
            // LangType::Custom(t) => t.as_str(),
            SemanticType::Ref(r#type) => {
                let type_name = r#type.to_string();
                format!("ref {}", type_name)
            }
            SemanticType::Array(inner_type, size) => {
                format!("[{}; {}]", inner_type.to_string(), size)
            }
        };

        write!(f, "{}", value)
    }
}

impl From<String> for SemanticType {
    fn from(value: String) -> Self {
        match value.as_str() {
            "void" => SemanticType::Void,
            "i8" => SemanticType::I8,
            "i16" => SemanticType::I16,
            "u32" => SemanticType::U32,
            "i32" => SemanticType::I32,
            "u8" => SemanticType::U8,
            "u16" => SemanticType::U16,
            "u64" => SemanticType::U64,
            "i64" => SemanticType::I64,
            "f32" => SemanticType::F32,
            "f64" => SemanticType::F64,
            "bool" => SemanticType::Bool,
            "char" => SemanticType::Char,
            "string" => SemanticType::String,
            "range" => SemanticType::Range,
            "any" => SemanticType::Any,
            _ => SemanticType::Any,
        }
    }
}

impl From<&str> for SemanticType {
    fn from(value: &str) -> Self {
        match value {
            "void" => SemanticType::Void,
            "i8" => SemanticType::I8,
            "i16" => SemanticType::I16,
            "u32" => SemanticType::U32,
            "i32" => SemanticType::I32,
            "u8" => SemanticType::U8,
            "u16" => SemanticType::U16,
            "u64" => SemanticType::U64,
            "i64" => SemanticType::I64,
            "f32" => SemanticType::F32,
            "f64" => SemanticType::F64,
            "bool" => SemanticType::Bool,
            "char" => SemanticType::Char,
            "string" => SemanticType::String,
            "range" => SemanticType::Range,
            "any" => SemanticType::Any,
            _ => SemanticType::Any,
        }
    }
}
