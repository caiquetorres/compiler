use super::semantic_type::SemanticType;

#[derive(Clone, Debug)]
pub enum Symbol {
    Variable {
        name: String,
        symbol_type: SemanticType,
    },
    Parameter {
        name: String,
        symbol_type: SemanticType,
    },
    Function {
        name: String,
        symbol_type: SemanticType,
    },
    Type {
        name: String,
    },
}

impl Symbol {
    pub fn get_name(&self) -> String {
        match &self {
            Self::Variable { name, .. } => name.clone(),
            Self::Function { name, .. } => name.clone(),
            Self::Parameter { name, .. } => name.clone(),
            Self::Type { name } => name.clone(),
        }
    }
}
