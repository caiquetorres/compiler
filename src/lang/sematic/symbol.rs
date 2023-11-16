use std::fmt::{self, Display, Formatter};

#[derive(PartialEq, Eq, Hash, Debug, Clone)]
pub(super) enum SymbolKind {
    Variable,
    Constant,
    Type,
    Parameter,
    Function(Vec<String>),
}

impl Display for SymbolKind {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Self::Variable => write!(f, "variable"),
            Self::Constant => write!(f, "constant"),
            Self::Type => write!(f, "type"),
            Self::Parameter => write!(f, "parameter"),
            Self::Function(_) => write!(f, "function"),
        }
    }
}

#[derive(PartialEq, Eq, Hash, Debug, Clone)]
pub(super) struct Symbol {
    pub name: String,
    pub kind: SymbolKind,
    pub symbol_type: Option<String>,
}

impl Symbol {
    pub fn new(name: &str, kind: SymbolKind, symbol_type: Option<&str>) -> Self {
        Self {
            name: String::from(name),
            kind,
            symbol_type: symbol_type.map(|s| String::from(s)),
        }
    }
}
