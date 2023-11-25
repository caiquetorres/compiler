use crate::lang::lexer::token::Token;

#[derive(Clone, Debug)]
pub enum SyntaxType {
    Simple {
        identifier: Token,
    },
    Array {
        r#type: Box<SyntaxType>,
        size: Token,
    },
    Reference {
        inner_type: Box<SyntaxType>,
    },
}

impl SyntaxType {
    pub fn new_simple(identifier: Token) -> Self {
        Self::Simple { identifier }
    }

    pub fn new_array(r#type: SyntaxType, size: Token) -> Self {
        Self::Array {
            r#type: Box::new(r#type),
            size,
        }
    }

    pub fn new_reference(inner_type: SyntaxType) -> Self {
        Self::Reference {
            inner_type: Box::new(inner_type),
        }
    }
}

impl ToString for SyntaxType {
    fn to_string(&self) -> String {
        match &self {
            Self::Simple { identifier } => identifier.value.clone(),
            Self::Reference { inner_type } => format!("ref {}", inner_type.to_string()),
            Self::Array { r#type, size } => {
                format!("[{}; {}]", r#type.to_string(), size.value.clone())
            }
        }
    }
}
