use std::fmt::{self, Display, Formatter};

pub enum LangType {
    Void,
    I8,
    I16,
    I32,
    I64,
    U8,
    U16,
    U32,
    U64,
    F32,
    F64,
    Bool,
    Char,
    String,
    Array(Box<LangType>, usize),
}

// Implement the Display trait for LangType
impl Display for LangType {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let type_str = match self {
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
            LangType::Array(inner_type, size) => {
                return write!(f, "[{}; {}]", inner_type, size);
            }
        };
        write!(f, "{}", type_str)
    }
}
