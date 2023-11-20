use super::symbol::Symbol;

use std::{collections::HashMap, rc::Rc};

#[derive(Clone)]
pub(super) struct Scope {
    is_loop: bool,
    function_return_type: Option<String>,
    pub parent: Option<Rc<Scope>>,
    pub symbol_table: HashMap<String, Symbol>,
}

impl Scope {
    pub fn new(
        parent: Option<Rc<Scope>>,
        is_loop: bool,
        function_return_type: Option<String>,
    ) -> Self {
        Self {
            is_loop,
            parent,
            symbol_table: HashMap::new(),
            function_return_type,
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
