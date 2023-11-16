#[derive(PartialEq, Eq, Hash, Debug, Clone)]
pub enum SymbolKind {
    Variable,
    Constant,
    Type,
    Parameter,
    Function,
}

#[derive(PartialEq, Eq, Hash, Debug, Clone)]
pub struct Symbol {
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
