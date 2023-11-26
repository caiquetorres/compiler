use crate::lang::{
    lexer::token::Token,
    position::{Position, Positioned},
};

#[derive(Clone, Debug)]
pub enum SyntaxType {
    Simple {
        identifier: Token,
    },
    Array {
        r#type: Box<SyntaxType>,
        size: Token,
        position: Position,
    },
    Reference {
        inner_type: Box<SyntaxType>,
        position: Position,
    },
    Function {
        params: Vec<SyntaxType>,
        r#type: Box<SyntaxType>,
        position: Position,
    },
}

impl SyntaxType {
    pub fn new_simple(identifier: Token) -> Self {
        Self::Simple { identifier }
    }

    pub fn new_array(r#type: SyntaxType, size: Token, position: Position) -> Self {
        Self::Array {
            r#type: Box::new(r#type),
            size,
            position,
        }
    }

    pub fn new_function(params: Vec<SyntaxType>, r#type: SyntaxType, position: Position) -> Self {
        Self::Function {
            params,
            r#type: Box::new(r#type),
            position,
        }
    }

    pub fn new_reference(inner_type: SyntaxType, position: Position) -> Self {
        Self::Reference {
            inner_type: Box::new(inner_type),
            position,
        }
    }
}

impl Positioned for SyntaxType {
    fn get_position(&self) -> Position {
        match &self {
            Self::Simple { identifier } => identifier.position,
            Self::Reference { position, .. } => position.clone(),
            Self::Array { position, .. } => position.clone(),
            Self::Function { position, .. } => position.clone(),
        }
    }
}

impl ToString for SyntaxType {
    fn to_string(&self) -> String {
        match &self {
            Self::Simple { identifier } => identifier.value.clone(),
            Self::Reference { inner_type, .. } => format!("ref {}", inner_type.to_string()),
            Self::Array { r#type, size, .. } => {
                format!("[{}; {}]", r#type.to_string(), size.value.clone())
            }
            Self::Function { params, r#type, .. } => {
                let mut str = String::from("(");

                for (index, param) in params.iter().enumerate() {
                    str.push_str(&param.to_string());

                    if index != params.len() - 1 {
                        str.push_str(", ");
                    }
                }

                str.push_str(&format!("): {}", r#type.to_string()));

                str
            }
        }
    }
}
