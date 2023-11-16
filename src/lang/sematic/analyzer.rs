use crate::lang::syntax::lexer::token_kind::TokenKind;
use crate::lang::syntax::parser::compilation_unit::CompilationUnit;
use crate::lang::syntax::parser::{
    expressions::{expression::Expression, literal::Literal},
    parser::Parser,
    shared::block::Block,
    statements::{
        assignment::Assignment, r#const::Const, r#let::Let, r#return::Return, statement::Statement,
    },
    top_level_statements::{function::Function, top_level_statement::TopLevelStatement},
};
use std::collections::HashMap;
use std::rc::Rc;

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

    pub fn get(&self, id: &str) -> Option<&Symbol> {
        self.symbols
            .get(id)
            .or_else(|| self.parent.as_ref().and_then(|p| p.get(id)))
    }

    pub fn contains(&self, id: &str) -> bool {
        self.symbols.contains_key(id) || self.parent.as_ref().map_or(false, |p| p.contains(id))
    }

    pub fn display(&self) {
        for el in &self.symbols {
            println!("{:?}", el);
        }
        println!("");
    }
}

pub struct Analyzer {
    ast: CompilationUnit,
}

impl Analyzer {
    pub fn from_ast(ast: CompilationUnit) -> Self {
        Self { ast }
    }

    pub fn from_code(code: &str) -> Result<Self, String> {
        let mut parser = Parser::from_code(code);
        let ast = parser.parse()?;

        Ok(Self { ast })
    }

    pub fn analyze(&mut self) -> Result<(), String> {
        let mut root_table = SymbolTable::new(None);

        root_table.insert("void", Symbol::new("void", SymbolKind::Type, None));
        root_table.insert("i32", Symbol::new("i32", SymbolKind::Type, None));
        root_table.insert("bool", Symbol::new("bool", SymbolKind::Type, None));
        root_table.insert("char", Symbol::new("char", SymbolKind::Type, None));
        root_table.insert("string", Symbol::new("string", SymbolKind::Type, None));

        for statement in &self.ast.statements {
            self.analyze_top_level_statement(statement, &mut root_table)?;
        }

        Ok(())
    }

    fn analyze_top_level_statement(
        &self,
        statement: &TopLevelStatement,
        table: &mut SymbolTable,
    ) -> Result<(), String> {
        match statement {
            TopLevelStatement::Function(function) => {
                let function_name = function.identifier.name.clone();
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
                        let return_type_name = return_type.name.clone();
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
                    .map_or("void", |id| &id.name[..]);

                table.insert(
                    &function_name,
                    Symbol::new(&function_name, SymbolKind::Function, Some(type_name)),
                );

                self.analyze_function(function, table)?;
            }
        }

        Ok(())
    }

    fn analyze_function(
        &self,
        function: &Function,
        parent_table: &SymbolTable,
    ) -> Result<(), String> {
        let rc = Rc::new(parent_table.clone());
        let mut table = SymbolTable::new(Some(rc));

        for param in &function.params_declaration.0 {
            let param_name = param.identifier.name.clone();
            let line = param.identifier.token.position.line;
            let column = param.identifier.token.position.column;

            if table.contains(&param_name) {
                return Err(format!(
                    "Duplicated parameter found: {} at Line {} and at Column {}",
                    param_name, line, column
                ));
            }

            let param_type = param.type_identifier.name.clone();
            let line = param.type_identifier.token.position.line;
            let column = param.type_identifier.token.position.column;

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
            self.analyze_statement(function, statement, &mut table)?;
        }

        Ok(())
    }

    fn analyze_statement(
        &self,
        function: &Function,
        statement: &Statement,
        table: &mut SymbolTable,
    ) -> Result<(), String> {
        match statement {
            Statement::Block(block) => self.analyze_block(function, block, table),
            Statement::Let(r#let) => self.analyze_let_statement(r#let, table),
            Statement::Const(r#const) => self.analyze_const_statement(r#const, table),
            Statement::Assignment(assignment) => {
                self.analyze_assignment_statement(assignment, table)
            }
            _ => Ok(()),
        }
    }

    fn analyze_block(
        &self,
        function: &Function,
        block: &Block,
        parent_table: &SymbolTable,
    ) -> Result<(), String> {
        let rc = Rc::new(parent_table.clone());
        let mut local_table = SymbolTable::new(Some(rc));

        for statement in &block.statements {
            self.analyze_statement(function, statement, &mut local_table)?;
        }

        Ok(())
    }

    fn analyze_let_statement(&self, r#let: &Let, table: &mut SymbolTable) -> Result<(), String> {
        match r#let {
            Let::WithoutValue(identifier, return_type) => {
                let variable_name = identifier.name.clone();

                let line = identifier.token.position.line;
                let column = identifier.token.position.column;

                if table.contains(&variable_name) {
                    return Err(format!(
                        "Duplicated identifier found: {} at Line {} and at Column {}",
                        variable_name, line, column
                    ));
                }

                let return_type_name = return_type.name.clone();

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
            Let::WithValue(identifier, return_type, expression) => {
                let variable_name = identifier.name.clone();

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
                        let return_type_name = self.analyze_expression(expression, table)?;

                        table.insert(
                            &variable_name,
                            Symbol::new(
                                &variable_name,
                                SymbolKind::Variable,
                                Some(&return_type_name),
                            ),
                        );
                    }
                    Some(return_type) => {
                        let return_type_name = return_type.name.clone();

                        let line = return_type.token.position.line;
                        let column = return_type.token.position.column;

                        if !table.contains(&return_type_name) {
                            return Err(format!(
                                "Type not found: {} at Line {} and at Column {}",
                                return_type_name, line, column
                            ));
                        }

                        let expression_return_type_name =
                            self.analyze_expression(expression, table)?;

                        if expression_return_type_name != return_type_name {
                            return Err(format!(
                                "Type mismatch, expected: {}, found: {}",
                                return_type_name, expression_return_type_name
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
        &self,
        r#const: &Const,
        table: &mut SymbolTable,
    ) -> Result<(), String> {
        let variable_name = r#const.identifier.name.clone();

        let line = r#const.identifier.token.position.line;
        let column = r#const.identifier.token.position.column;

        if table.contains(&variable_name) {
            return Err(format!(
                "Duplicated identifier found: {} at Line {} and at Column {}",
                variable_name, line, column
            ));
        }

        match &r#const.type_identifier {
            None => {
                let return_type_name = self.analyze_expression(&r#const.expression, table)?;

                table.insert(
                    &variable_name,
                    Symbol::new(
                        &variable_name,
                        SymbolKind::Constant,
                        Some(&return_type_name),
                    ),
                );
            }
            Some(return_type) => {
                let return_type_name = return_type.name.clone();

                let line = return_type.token.position.line;
                let column = return_type.token.position.column;

                if !table.contains(&return_type_name) {
                    return Err(format!(
                        "Type not found: {} at Line {} and at Column {}",
                        return_type_name, line, column
                    ));
                }

                let expression_return_type_name =
                    self.analyze_expression(&r#const.expression, table)?;

                if expression_return_type_name != return_type_name {
                    return Err(format!(
                        "Type mismatch, expected: {}, found: {}",
                        return_type_name, expression_return_type_name
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

        Ok(())
    }

    fn analyze_assignment_statement(
        &self,
        assignment: &Assignment,
        table: &SymbolTable,
    ) -> Result<(), String> {
        let identifier_name = assignment.identifier.name.clone();
        let line = assignment.identifier.token.position.line;
        let column = assignment.identifier.token.position.column;

        if !table.contains(&identifier_name) {
            return Err(format!(
                "Identifier not found: {} at Line {} and at Column {}",
                identifier_name, line, column
            ));
        }

        let symbol = table.get(&identifier_name).unwrap();
        let variable_type = symbol.symbol_type.as_ref().unwrap().clone();
        let expression_return_type = self.analyze_expression(&assignment.expression, table)?;

        if let SymbolKind::Variable = &symbol.kind {
            match assignment.operator.token.kind {
                TokenKind::Equals => {}
                TokenKind::PlusEquals
                | TokenKind::MinusEquals
                | TokenKind::StarEquals
                | TokenKind::SlashEquals
                | TokenKind::ModEquals
                | TokenKind::AmpersandEquals
                | TokenKind::PipeEquals
                | TokenKind::CircumflexEquals => {
                    if variable_type != "i32" {
                        return Err(format!(
                            "Type mismatch in '{}': expected 'i32' for the left-hand side, found '{}'",
                            assignment.operator.token.value, variable_type
                        ));
                    }

                    if expression_return_type != "i32" {
                        return Err(format!(
                            "Type mismatch in '{}': expected 'i32' for the right-hand side, found '{}'",
                            assignment.operator.token.value,
                            expression_return_type
                        ));
                    }
                }
                _ => {
                    return Err(format!(
                        "Type mismatch in assignment: '{}'",
                        identifier_name,
                    ))
                }
            };

            if variable_type != expression_return_type {
                return Err(format!(
                    "Type mismatch in '{}': expected '{}', found '{}'",
                    identifier_name, variable_type, expression_return_type
                ));
            }

            Ok(())
        } else {
            Err(format!(
                "Only variables can change its value, assignment not allowed: {} at Line {} and at Column {}",
                identifier_name, line, column
            ))
        }
    }

    fn analyze_expression(
        &self,
        expression: &Expression,
        table: &SymbolTable,
    ) -> Result<String, String> {
        match expression {
            Expression::Identifier(identifier) => {
                let identifier_name = identifier.name.clone();
                let line = identifier.token.position.line;
                let column = identifier.token.position.column;

                match table.get(&identifier_name) {
                    None => Err(format!(
                        "Identifier not found: {} at Line {} and at Column {}",
                        identifier_name, line, column
                    )),
                    Some(symbol) => Ok(symbol.symbol_type.as_ref().unwrap().clone()),
                }
            }
            Expression::Literal(literal) => match literal {
                Literal::Number(_) => Ok("i32".to_string()),
                Literal::Boolean(_) => Ok("bool".to_string()),
                Literal::Char(_) => Ok("char".to_string()),
                Literal::String(_) => Ok("string".to_string()),
            },
            Expression::Parenthesized(parenthesize) => {
                self.analyze_expression(parenthesize.0.as_ref(), table)
            }
            Expression::Unary(unary) => {
                let return_type = self.analyze_expression(&unary.expression, table)?;

                match unary.operator.0.kind {
                    TokenKind::Plus | TokenKind::Minus | TokenKind::Tilde => {
                        if return_type != "i32" {
                            return Err(format!("Expected type 'i32', found '{}'", return_type,));
                        }

                        Ok("i32".to_string())
                    }
                    TokenKind::Exclamation => {
                        if return_type != "bool" {
                            return Err(format!("Expected type 'bool', found '{}'", return_type,));
                        }

                        Ok("bool".to_string())
                    }
                    _ => Err(format!("Mismatch types")),
                }
            }
            Expression::Binary(binary) => {
                let left_return_type = self.analyze_expression(&binary.left, table)?;
                let right_return_type = self.analyze_expression(&binary.right, table)?;

                match binary.operator.0.kind {
                    TokenKind::EqualsEquals => {
                        if left_return_type != right_return_type {
                            return Err(format!(
                                "Mismatched types for equality comparison: '{}' and '{}'",
                                left_return_type, right_return_type
                            ));
                        }
                        Ok("bool".to_string())
                    }
                    TokenKind::Plus
                    | TokenKind::Minus
                    | TokenKind::Star
                    | TokenKind::Slash
                    | TokenKind::Mod
                    | TokenKind::Ampersand
                    | TokenKind::Pipe
                    | TokenKind::Tilde
                    | TokenKind::Circumflex => {
                        if left_return_type != "i32" || right_return_type != "i32" {
                            return Err(format!(
                                "Expected 'i32' for both operands, found '{}' and '{}'",
                                left_return_type, right_return_type
                            ));
                        }
                        Ok("i32".to_string())
                    }
                    TokenKind::GreaterThan
                    | TokenKind::GreaterThanEquals
                    | TokenKind::LessThan
                    | TokenKind::LessThanEquals => {
                        if left_return_type != "i32" || right_return_type != "i32" {
                            return Err(format!(
                                "Expected 'i32' for both operands, found '{}' and '{}'",
                                left_return_type, right_return_type
                            ));
                        }
                        Ok("bool".to_string())
                    }
                    TokenKind::AmpersandAmpersand | TokenKind::PipePipe => {
                        if left_return_type != "bool" || right_return_type != "bool" {
                            return Err(format!(
                                "Expected 'bool' for both operands, found '{}' and '{}'",
                                left_return_type, right_return_type
                            ));
                        }
                        Ok("bool".to_string())
                    }
                    _ => Err(format!("Mismatch types")),
                }
            }
            _ => Err("Error".to_string()),
        }
    }

    fn analyze_return_statement(
        &mut self,
        function: &Function,
        r#return: &Return,
        table: &SymbolTable,
    ) -> Result<(), String> {
        Ok(())
    }
}
