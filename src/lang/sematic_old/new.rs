use std::{collections::HashMap, rc::Rc};

use super::lang_type::LangType;

pub enum Symbol {
    Variable { name: String, symbol_type: LangType },
    Parameter { name: String, symbol_type: LangType },
    Const { name: String, symbol_type: LangType },
    Function { name: String, params: Vec<LangType> },
    Type { name: String },
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

pub trait Scope {
    fn insert(&mut self, symbol: Symbol);

    fn get(&self, name: &str) -> Option<&Symbol>;
}

pub struct GlobalScope {
    symbol_table: HashMap<String, Symbol>,
}

impl GlobalScope {
    pub fn new() -> Self {
        Self {
            symbol_table: HashMap::new(),
        }
    }
}

impl Scope for GlobalScope {
    fn insert(&mut self, symbol: Symbol) {
        self.symbol_table.insert(symbol.get_name(), symbol);
    }

    fn get(&self, name: &str) -> Option<&Symbol> {
        self.symbol_table.get(name)
    }
}

pub struct LocalScope {
    is_loop: bool,
    parent: Rc<dyn Scope>,
    return_type: Option<LangType>,
    symbol_table: HashMap<String, Symbol>,
}

impl LocalScope {
    pub fn new(parent: Rc<dyn Scope>) -> Self {
        Self {
            parent,
            is_loop: false,
            return_type: None,
            symbol_table: HashMap::new(),
        }
    }
}

impl Scope for LocalScope {
    fn insert(&mut self, symbol: Symbol) {
        self.symbol_table.insert(symbol.get_name(), symbol);
    }

    fn get(&self, name: &str) -> Option<&Symbol> {
        self.symbol_table.get(name)
    }
}

fn main() {
    let global = GlobalScope::new();
    let local1 = LocalScope::new(Rc::new(global));
    let local2 = LocalScope::new(Rc::new(local1));
}
