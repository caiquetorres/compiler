use crate::lang::syntax::{lexer::token::Token, tree_display::TreeDisplay};

#[derive(Clone)]
pub struct ArrayProps {
    // TODO: Improve this struct name
    pub size: Token,
}

#[derive(Clone)]
pub enum Type {
    Simple { identifier: Token },
    Array { r#type: Box<Type>, size: Token },
    Reference { inner_type: Box<Type> },
}

impl Type {
    pub fn new_simple(identifier: Token) -> Self {
        Self::Simple { identifier }
    }

    pub fn new_array(r#type: Type, size: Token) -> Self {
        Self::Array {
            r#type: Box::new(r#type),
            size,
        }
    }

    pub fn new_reference(inner_type: Type) -> Self {
        Self::Reference {
            inner_type: Box::new(inner_type),
        }
    }
}

impl TreeDisplay for Type {
    fn display(&self, layer: usize) {
        print!("{}", " ".repeat(layer));

        match &self {
            Self::Simple { identifier } => {
                // print!("array ");
                // identifier.display(0);
            }
            Self::Array { r#type, size } => {
                // print!("array ");
                // identifier.display(0);
            }
            Self::Reference { inner_type } => {
                print!("ref ");
                inner_type.display(0);
            }
        }
    }
}
