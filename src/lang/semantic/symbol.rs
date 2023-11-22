use super::lang_type::LangType;

#[derive(Clone, Debug)]
pub enum Symbol {
    Variable {
        name: String,
        symbol_type: LangType,
    },
    Parameter {
        name: String,
        symbol_type: LangType,
    },
    Const {
        name: String,
        symbol_type: LangType,
    },
    Function {
        name: String,
        symbol_type: LangType,
        params: Vec<LangType>,
    },
    Type {
        name: String,
    },
}

impl Symbol {
    pub fn get_name(&self) -> String {
        match &self {
            Self::Variable { name, .. } => name.clone(),
            Self::Const { name, .. } => name.clone(),
            Self::Function { name, .. } => name.clone(),
            Self::Parameter { name, .. } => name.clone(),
            Self::Type { name } => name.clone(),
        }
    }
}
