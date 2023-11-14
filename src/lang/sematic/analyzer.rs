use std::{collections::HashMap, rc::Rc};

use crate::lang::syntax::parser::{
    expressions::expression::Expression,
    parser::Parser,
    shared::block::Block,
    statements::{r#const::Const, r#let::Let, statement::Statement},
    top_level_statements::{function::Function, top_level_statement::TopLevelStatement},
};

#[derive(PartialEq, Eq, Hash, Debug, Clone)]
enum SymbolKind {
    Variable,
    Constant,
    Type,
    Parameter,
    Function,
}

#[derive(PartialEq, Eq, Hash, Debug, Clone)]
struct Symbol {
    name: String,
    kind: SymbolKind,
    symbol_type: Option<String>,
}

impl Symbol {
    fn new(name: &str, kind: SymbolKind, symbol_type: Option<&str>) -> Self {
        Self {
            name: String::from(name),
            kind,
            symbol_type: symbol_type.map(|s| String::from(s)),
        }
    }
}

#[derive(Clone)]
struct SymbolTable {
    parent: Option<Rc<SymbolTable>>,
    symbols: HashMap<String, Symbol>,
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

    pub fn contains(&self, id: &str) -> bool {
        if !self.symbols.contains_key(id) {
            return match &self.parent {
                None => false,
                Some(p) => p.contains(id),
            };
        }
        return true;
    }

    pub fn display(&self) {
        for el in &self.symbols {
            println!("{:?}", el);
        }
        println!("");
    }
}

pub struct Analyzer {
    parser: Parser,
}

impl Analyzer {
    pub fn new(code: &str) -> Self {
        Self {
            parser: Parser::new(code),
        }
    }

    pub fn analyze(&mut self) -> Result<(), String> {
        let ast = self.parser.parse()?;
        let mut root_table = SymbolTable::new(None);

        root_table.insert("i32", Symbol::new("i32", SymbolKind::Type, None));
        root_table.insert("i64", Symbol::new("i64", SymbolKind::Type, None));

        for statement in &ast.statements {
            self.analyze_top_level_statement(statement, &mut root_table)?;
        }

        Ok(())
    }

    fn analyze_top_level_statement(
        &mut self,
        statement: &TopLevelStatement,
        table: &mut SymbolTable,
    ) -> Result<(), String> {
        match statement {
            TopLevelStatement::Function(function) => {
                let function_name = function.identifier.token.value.clone();
                let line = function.identifier.token.position.line;
                let column = function.identifier.token.position.column;

                if table.contains(&function_name) {
                    return Err(format!(
                        "Duplicated identifier found: {} at Line {} and at Column {}",
                        function_name, line, column
                    ));
                }

                match &function.type_identifier {
                    None => {
                        table.insert(
                            &function_name,
                            Symbol::new(&function_name, SymbolKind::Variable, None),
                        );
                    }
                    Some(return_type) => {
                        let return_type_name = return_type.token.value.clone();
                        let line = return_type.token.position.line;
                        let column = return_type.token.position.column;

                        if !table.contains(&return_type_name) {
                            return Err(format!(
                                "Type not found: {} at Line {} and at Column {}",
                                return_type_name, line, column
                            ));
                        }

                        table.insert(
                            &function_name,
                            Symbol::new(
                                &function_name,
                                SymbolKind::Variable,
                                Some(&return_type_name),
                            ),
                        );
                    }
                };

                let type_name = function
                    .type_identifier
                    .as_ref()
                    .map(|id| &id.token.value[..]);

                table.insert(
                    &function_name,
                    Symbol::new(&function_name, SymbolKind::Function, type_name),
                );

                self.analyze_function(function, table)?;
            }
        }

        Ok(())
    }

    fn analyze_function(
        &mut self,
        function: &Function,
        parent_table: &SymbolTable,
    ) -> Result<(), String> {
        let rc = Rc::new(parent_table.clone());
        let mut table = SymbolTable::new(Some(rc));

        for param in &function.params_declaration.0 {
            let param_name = param.0.token.value.clone();
            let line = param.0.token.position.line;
            let column = param.0.token.position.column;

            if table.contains(&param_name) {
                return Err(format!(
                    "Duplicated parameter found: {} at Line {} and at Column {}",
                    param_name, line, column
                ));
            }

            let param_type = param.1.token.value.clone();
            let line = param.1.token.position.line;
            let column = param.1.token.position.column;

            if !table.contains(&param_type) {
                return Err(format!(
                    "Type not found: {} at Line {} and at Column {}",
                    param_type, line, column
                ));
            }

            table.insert(
                &param_name.clone(),
                Symbol::new(&param_name, SymbolKind::Parameter, Some(&param_type)),
            )
        }

        for statement in &function.block.statements {
            self.analyze_statement(statement, &mut table)?;
        }

        table.display();

        Ok(())
    }

    fn analyze_statement(
        &mut self,
        statement: &Statement,
        table: &mut SymbolTable,
    ) -> Result<(), String> {
        match statement {
            Statement::Block(block) => self.analyze_block(block, table),
            Statement::Let(r#let) => self.analyze_let_statement(r#let, table),
            Statement::Const(r#const) => self.analyze_const_statement(r#const, table),
            _ => Ok(()),
        }
    }

    fn analyze_block(&mut self, block: &Block, parent_table: &SymbolTable) -> Result<(), String> {
        let rc = Rc::new(parent_table.clone());
        let mut local_table = SymbolTable::new(Some(rc));

        for statement in &block.statements {
            self.analyze_statement(statement, &mut local_table)?;
        }

        Ok(())
    }

    fn analyze_let_statement(
        &mut self,
        r#let: &Let,
        table: &mut SymbolTable,
    ) -> Result<(), String> {
        match r#let {
            Let::WithoutValue(identifier, return_type) => {
                let variable_name = identifier.token.value.clone();

                let line = identifier.token.position.line;
                let column = identifier.token.position.column;

                if table.contains(&variable_name) {
                    return Err(format!(
                        "Duplicated identifier found: {} at Line {} and at Column {}",
                        variable_name, line, column
                    ));
                }

                let return_type_name = return_type.token.value.clone();

                let line = return_type.token.position.line;
                let column = return_type.token.position.column;

                if !table.contains(&return_type_name) {
                    return Err(format!(
                        "Type not found: {} at Line {} and at Column {}",
                        return_type_name, line, column
                    ));
                }

                table.insert(
                    &variable_name,
                    Symbol::new(
                        &variable_name,
                        SymbolKind::Variable,
                        Some(&return_type_name),
                    ),
                );
            }
            Let::WithValue(identifier, return_type, _, _) => {
                let variable_name = identifier.token.value.clone();

                let line = identifier.token.position.line;
                let column = identifier.token.position.column;

                if table.contains(&variable_name) {
                    return Err(format!(
                        "Duplicated identifier found: {} at Line {} and at Column {}",
                        variable_name, line, column
                    ));
                }

                match return_type {
                    None => {
                        let return_type_name = "i32"; // TODO: Infer type

                        table.insert(
                            &variable_name,
                            Symbol::new(
                                &variable_name,
                                SymbolKind::Variable,
                                Some(return_type_name),
                            ),
                        );
                    }
                    Some(return_type) => {
                        let return_type_name = return_type.token.value.clone();

                        let line = return_type.token.position.line;
                        let column = return_type.token.position.column;

                        if !table.contains(&return_type_name) {
                            return Err(format!(
                                "Type not found: {} at Line {} and at Column {}",
                                return_type_name, line, column
                            ));
                        }

                        table.insert(
                            &variable_name,
                            Symbol::new(
                                &variable_name,
                                SymbolKind::Variable,
                                Some(&return_type_name),
                            ),
                        );
                    }
                }
            }
        }

        Ok(())
    }

    fn analyze_const_statement(
        &mut self,
        r#const: &Const,
        table: &mut SymbolTable,
    ) -> Result<(), String> {
        match r#const {
            Const::WithoutValue(identifier, return_type) => {
                let variable_name = identifier.token.value.clone();

                let line = identifier.token.position.line;
                let column = identifier.token.position.column;

                if table.contains(&variable_name) {
                    return Err(format!(
                        "Duplicated identifier found: {} at Line {} and at Column {}",
                        variable_name, line, column
                    ));
                }

                let return_type_name = return_type.token.value.clone();

                let line = return_type.token.position.line;
                let column = return_type.token.position.column;

                if !table.contains(&return_type_name) {
                    return Err(format!(
                        "Type not found: {} at Line {} and at Column {}",
                        return_type_name, line, column
                    ));
                }

                table.insert(
                    &variable_name,
                    Symbol::new(
                        &variable_name,
                        SymbolKind::Constant,
                        Some(&return_type_name),
                    ),
                );
            }
            Const::WithValue(identifier, return_type, _, _) => {
                let variable_name = identifier.token.value.clone();

                let line = identifier.token.position.line;
                let column = identifier.token.position.column;

                if table.contains(&variable_name) {
                    return Err(format!(
                        "Duplicated identifier found: {} at Line {} and at Column {}",
                        variable_name, line, column
                    ));
                }

                match return_type {
                    None => {
                        let return_type_name = "i32"; // TODO: Infer type

                        table.insert(
                            &variable_name,
                            Symbol::new(
                                &variable_name,
                                SymbolKind::Constant,
                                Some(return_type_name),
                            ),
                        );
                    }
                    Some(return_type) => {
                        let return_type_name = return_type.token.value.clone();

                        let line = return_type.token.position.line;
                        let column = return_type.token.position.column;

                        if !table.contains(&return_type_name) {
                            return Err(format!(
                                "Type not found: {} at Line {} and at Column {}",
                                return_type_name, line, column
                            ));
                        }

                        table.insert(
                            &variable_name,
                            Symbol::new(
                                &variable_name,
                                SymbolKind::Variable,
                                Some(&return_type_name),
                            ),
                        );
                    }
                }
            }
        }

        Ok(())
    }

    fn analyze_expression(
        &mut self,
        expression: &Expression,
        table: &SymbolTable,
    ) -> Result<String, String> {
        todo!()
    }
}
