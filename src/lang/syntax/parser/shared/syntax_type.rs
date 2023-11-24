use crate::lang::{lexer::token::Token, syntax::tree_display::TreeDisplay};

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

impl TreeDisplay for SyntaxType {
    fn display(&self, layer: usize) {
        print!("{}", " ".repeat(layer));

        // match &self {
        //     Self::Simple { identifier } => {
        //         // print!("array ");
        //         // identifier.display(0);
        //     }
        //     Self::Array { r#type, size } => {
        //         // print!("array ");
        //         // identifier.display(0);
        //     }
        //     Self::Reference { inner_type } => {
        //         print!("ref ");
        //         inner_type.display(0);
        //     }
        // }
    }
}
