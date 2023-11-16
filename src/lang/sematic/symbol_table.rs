use std::{collections::HashMap, rc::Rc};

use super::symbol::Symbol;

#[derive(Clone)]
pub struct SymbolTable {
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

    pub fn insert(&mut self, id: &str, symbol: Symbol) {
        self.symbols.insert(String::from(id), symbol);
    }

    pub fn get(&self, id: &str) -> Option<&Symbol> {
        self.symbols
            .get(id)
            .or_else(|| self.parent.as_ref().and_then(|p| p.get(id)))
    }

    pub fn contains(&self, id: &str) -> bool {
        self.symbols.contains_key(id) || self.parent.as_ref().map_or(false, |p| p.contains(id))
    }

    #[allow(dead_code)]
    pub fn display(&self) {
        for el in &self.symbols {
            println!("{:?}", el);
        }
        println!("");
    }
}
