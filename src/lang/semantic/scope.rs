use std::{cell::RefCell, collections::HashMap, rc::Rc};

use super::{lang_type::LangType, symbol::Symbol};

#[derive(Clone, Debug)]
pub struct Func {
    pub name: String,
    pub return_type: LangType,
}

#[derive(Clone, Debug)]
pub struct Scope {
    is_loop: bool,
    parent: Option<Rc<RefCell<Scope>>>,
    function: Option<Func>,
    symbol_table: HashMap<String, Symbol>,
}

impl Scope {
    pub fn global() -> Self {
        Self {
            parent: None,
            is_loop: false,
            function: None,
            symbol_table: HashMap::new(),
        }
    }

    pub fn new(parent: Rc<RefCell<Scope>>, is_loop: bool, function: Option<Func>) -> Self {
        Self {
            parent: Some(parent),
            is_loop,
            function,
            symbol_table: HashMap::new(),
        }
    }

    pub fn is_loop(&self) -> bool {
        self.is_loop || matches!(&self.parent, Some(parent) if parent.borrow().is_loop())
    }

    pub fn insert(&mut self, symbol: Symbol) {
        self.symbol_table.insert(symbol.get_name().clone(), symbol);
    }

    pub fn get(&self, name: &str) -> Option<Symbol> {
        self.symbol_table.get(name).cloned().or_else(|| {
            self.parent
                .as_ref()
                .and_then(|parent| parent.borrow().get(name))
        })
    }

    pub fn get_function_name(&self) -> Option<String> {
        self.function.clone().map(|v| v.name.clone()).or_else(|| {
            self.parent
                .as_ref()
                .and_then(|p| p.borrow().get_function_name())
        })
    }

    pub fn get_return_type(&self) -> Option<LangType> {
        self.function
            .clone()
            .map(|v| v.return_type.clone())
            .or_else(|| {
                self.parent
                    .as_ref()
                    .and_then(|p| p.borrow().get_return_type())
            })
    }
}
