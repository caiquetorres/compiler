use super::symbol::Symbol;

use std::{collections::HashMap, rc::Rc};

#[derive(Clone, Debug)]
pub struct Scope {
    is_loop: bool,
    function_return_type: Option<String>,
    pub parent: Option<Rc<Scope>>,
    pub symbol_table: HashMap<String, Symbol>,
}

impl Scope {
    pub fn global() -> Self {
        Self {
            parent: None,
            is_loop: false,
            function_return_type: None,
            symbol_table: HashMap::new(),
        }
    }

    pub fn extend(scope: Scope) -> Self {
        Self { ..scope }
    }

    pub fn block(parent: Scope, is_loop: bool) -> Self {
        Self {
            parent: Some(Rc::new(parent)),
            is_loop,
            function_return_type: None,
            symbol_table: HashMap::new(),
        }
    }

    pub fn new(parent: Scope, is_loop: bool, function_return_type: Option<String>) -> Self {
        Self {
            parent: Some(Rc::new(parent)),
            is_loop,
            function_return_type,
            symbol_table: HashMap::new(),
        }
    }

    pub fn is_loop(&self) -> bool {
        self.is_loop || matches!(&self.parent, Some(parent) if parent.is_loop())
    }

    pub fn insert_symbol(&mut self, symbol: Symbol) {
        self.symbol_table.insert(symbol.name.clone(), symbol);
    }

    pub fn get_fun_return_type(&self) -> Option<String> {
        self.function_return_type
            .clone()
            .or_else(|| self.parent.as_ref().and_then(|p| p.get_fun_return_type()))
    }

    pub fn get_symbol(&self, name: &str) -> Option<&Symbol> {
        self.symbol_table
            .get(name)
            .or_else(|| self.parent.as_ref().and_then(|p| p.get_symbol(name)))
    }

    #[allow(dead_code)]
    pub fn display(&self) {
        for el in &self.symbol_table {
            println!("{:?}", el);
        }
        println!("");
    }
}
