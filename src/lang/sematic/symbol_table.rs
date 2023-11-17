use super::symbol::Symbol;

use std::{collections::HashMap, rc::Rc};

#[derive(Clone)]
pub(super) struct SymbolTable {
    pub parent: Option<Rc<SymbolTable>>,
    pub symbols: HashMap<String, Symbol>,
}

impl SymbolTable {
    pub fn new(parent: Option<Rc<SymbolTable>>) -> Self {
        Self {
            parent,
            symbols: HashMap::new(),
        }
    }

    pub fn insert(&mut self, symbol: Symbol) {
        self.symbols.insert(symbol.name.clone(), symbol);
    }

    pub fn get(&self, name: &str) -> Option<&Symbol> {
        self.symbols
            .get(name)
            .or_else(|| self.parent.as_ref().and_then(|p| p.get(name)))
    }

    pub fn contains(&self, name: &str) -> bool {
        self.symbols.contains_key(name) || self.parent.as_ref().map_or(false, |p| p.contains(name))
    }

    #[allow(dead_code)]
    pub fn display(&self) {
        for el in &self.symbols {
            println!("{:?}", el);
        }
        println!("");
    }
}
