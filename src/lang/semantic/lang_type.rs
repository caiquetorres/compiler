#[derive(Clone, Debug, PartialEq, Eq)]
pub enum LangType {
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
    //Array(Box<LangType>, usize),
}

impl LangType {
    pub fn number_type_precedence(v: Vec<LangType>) -> LangType {
        if v.contains(&LangType::Any) {
            LangType::Any
        } else if v.contains(&LangType::F64) {
            LangType::F64
        } else if v.contains(&LangType::F32) {
            LangType::F32
        } else if v.contains(&LangType::I64) {
            LangType::I64
        } else if v.contains(&LangType::U64) {
            LangType::U64
        } else if v.contains(&LangType::I32) {
            LangType::I32
        } else if v.contains(&LangType::U32) {
            LangType::U32
        } else if v.contains(&LangType::I16) {
            LangType::I16
        } else if v.contains(&LangType::U16) {
            LangType::U16
        } else if v.contains(&LangType::I8) {
            LangType::I8
        } else {
            LangType::U8
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
}

impl ToString for LangType {
    fn to_string(&self) -> String {
        match self {
            LangType::Void => "void",
            LangType::I8 => "i8",
            LangType::I16 => "i16",
            LangType::I32 => "i32",
            LangType::I64 => "i64",
            LangType::U8 => "u8",
            LangType::U16 => "u16",
            LangType::U32 => "u32",
            LangType::U64 => "u64",
            LangType::F32 => "f32",
            LangType::F64 => "f64",
            LangType::Bool => "bool",
            LangType::Char => "char",
            LangType::String => "string",
            LangType::Range => "range",
            LangType::Any => "any",
            // LangType::Custom(t) => t.as_str(),
            // LangType::Array(inner_type, size) => {
            //     return write!(f, "[{}; {}]", inner_type, size);
            // }
        }
        .to_string()
    }
}

impl From<String> for LangType {
    fn from(value: String) -> Self {
        match value.as_str() {
            "void" => LangType::Void,
            "i8" => LangType::I8,
            "i16" => LangType::I16,
            "u32" => LangType::U32,
            "i32" => LangType::I32,
            "u8" => LangType::U8,
            "u16" => LangType::U16,
            "u64" => LangType::U64,
            "i64" => LangType::I64,
            "f32" => LangType::F32,
            "f64" => LangType::F64,
            "bool" => LangType::Bool,
            "char" => LangType::Char,
            "string" => LangType::String,
            "range" => LangType::Range,
            "any" => LangType::Any,
            _ => LangType::Any,
        }
    }
}

impl From<&str> for LangType {
    fn from(value: &str) -> Self {
        match value {
            "void" => LangType::Void,
            "i8" => LangType::I8,
            "i16" => LangType::I16,
            "u32" => LangType::U32,
            "i32" => LangType::I32,
            "u8" => LangType::U8,
            "u16" => LangType::U16,
            "u64" => LangType::U64,
            "i64" => LangType::I64,
            "f32" => LangType::F32,
            "f64" => LangType::F64,
            "bool" => LangType::Bool,
            "char" => LangType::Char,
            "string" => LangType::String,
            "range" => LangType::Range,
            "any" => LangType::Any,
            _ => LangType::Any,
        }
    }
}
