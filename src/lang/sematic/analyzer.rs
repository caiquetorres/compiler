use uuid::Uuid;

use super::scope::Scope;
use super::symbol::{Symbol, SymbolKind};

use crate::lang::syntax::lexer::token_kind::TokenKind;
use crate::lang::syntax::parser::compilation_unit::CompilationUnit;
use crate::lang::syntax::parser::expressions::{expression::Expression, literal::Literal};
use crate::lang::syntax::parser::shared::{block::Block, function_call::FunctionCall};
use crate::lang::syntax::parser::statements::print::Print;
use crate::lang::syntax::parser::statements::r#break::Break;
use crate::lang::syntax::parser::statements::r#continue::Continue;
use crate::lang::syntax::parser::statements::r#return::Return;
use crate::lang::syntax::parser::statements::{
    assignment::Assignment, do_while::DoWhile, r#const::Const, r#for::For, r#if::If, r#let::Let,
    r#while::While, statement::Statement,
};
use crate::lang::syntax::parser::top_level_statements::{
    function::Function, top_level_statement::TopLevelStatement,
};

use std::collections::HashMap;

fn is_number(text: &str) -> bool {
    matches!(
        text,
        "i8" | "i16" | "i32" | "i64" | "u8" | "u16" | "u32" | "u64" | "f32" | "f64"
    )
}

fn is_integer(text: &str) -> bool {
    matches!(
        text,
        "i8" | "i16" | "i32" | "i64" | "u8" | "u16" | "u32" | "u64"
    )
}

fn number_type_precedence(v: Vec<&str>) -> String {
    if v.contains(&"f64") {
        return String::from("f64");
    }
    if v.contains(&"f32") {
        return String::from("f32");
    }
    if v.contains(&"i64") {
        return String::from("i64");
    }
    if v.contains(&"u64") {
        return String::from("u64");
    }
    if v.contains(&"i32") {
        return String::from("i32");
    }
    if v.contains(&"u32") {
        return String::from("u32");
    }
    if v.contains(&"i16") {
        return String::from("i16");
    }
    if v.contains(&"u16") {
        return String::from("u16");
    }
    if v.contains(&"i8") {
        return String::from("i8");
    }

    return String::from("u8");
}

pub struct Analyzer {
    ast: CompilationUnit,
}

impl Analyzer {
    pub fn from_ast(ast: CompilationUnit) -> Self {
        Self { ast }
    }

    #[cfg(test)]
    pub fn from_code(code: &str) -> Result<Self, String> {
        use crate::lang::syntax::parser::parser::Parser;

        let mut parser = Parser::from_code(code);
        let ast = parser.parse().unwrap();

        Ok(Self { ast })
    }

    pub fn analyze(&mut self) -> Result<HashMap<Uuid, Scope>, String> {
        let mut block_map: HashMap<Uuid, Scope> = HashMap::new();
        let mut root_scope = Scope::global();

        let default_types = [
            "void", "i8", "i16", "i32", "i64", "u8", "u16", "u32", "u64", "f32", "f64", "bool",
            "char", "string",
        ];

        for default_type in &default_types {
            root_scope.insert_symbol(Symbol::new(default_type, SymbolKind::Type, None));
        }

        for statement in &self.ast.statements {
            self.analyze_top_level_statement(statement, &mut root_scope, &mut block_map)?;
        }

        if root_scope.get_symbol("main").is_none() {
            return Err(format!("Missing main function",));
        }

        Ok(block_map)
    }

    fn analyze_top_level_statement(
        &self,
        statement: &TopLevelStatement,
        scope: &mut Scope,
        block_map: &mut HashMap<Uuid, Scope>,
    ) -> Result<(), String> {
        match statement {
            TopLevelStatement::Function(function) => {
                self.analyze_function_declaration(function, scope, block_map)?
            }
        }

        Ok(())
    }

    fn analyze_function_declaration(
        &self,
        function: &Function,
        global_scope: &mut Scope,
        block_map: &mut HashMap<Uuid, Scope>,
    ) -> Result<(), String> {
        let function_name = function.identifier.name.clone();
        let line = function.identifier.token.position.line;
        let column = function.identifier.token.position.column;

        if global_scope.get_symbol(&function_name).is_some() {
            return Err(format!(
                "Duplicated identifier found: {} at Line {} and at Column {}",
                function_name, line, column
            ));
        }

        match &function.type_identifier {
            None => {}
            Some(return_type) => {
                let return_type_name = return_type.name.clone();
                let line = return_type.token.position.line;
                let column = return_type.token.position.column;

                if !global_scope.get_symbol(&return_type_name).is_some() {
                    return Err(format!(
                        "Type not found: {} at Line {} and at Column {}",
                        return_type_name, line, column
                    ));
                }
            }
        };

        let params: Vec<String> = function
            .params_declaration
            .params
            .iter()
            .map(|param| param.type_identifier.name.clone())
            .collect();

        if function_name == "main" && params.len() != 0 {
            return Err(format!("Main function cannot have params",));
        }

        let type_name = function
            .type_identifier
            .as_ref()
            .map_or("void", |id| &id.name[..]);

        global_scope.insert_symbol(Symbol::new(
            &function_name,
            SymbolKind::Function(params),
            Some(type_name),
        ));

        self.analyze_function(function, &global_scope, block_map)?;

        Ok(())
    }

    fn analyze_function(
        &self,
        function: &Function,
        parent_scope: &Scope,
        block_map: &mut HashMap<Uuid, Scope>,
    ) -> Result<(), String> {
        let function_return_type_name = function
            .type_identifier
            .as_ref()
            .map_or("void".to_string(), |id| id.name.clone());

        let mut scope = Scope::new(parent_scope.clone(), false, Some(function_return_type_name));

        for param in &function.params_declaration.params {
            let param_name = param.identifier.name.clone();
            let line = param.identifier.token.position.line;
            let column = param.identifier.token.position.column;

            if scope.get_symbol(&param_name).is_some() {
                return Err(format!(
                    "Duplicated parameter found: {} at Line {} and at Column {}",
                    param_name, line, column
                ));
            }

            let param_type = param.type_identifier.name.clone();
            let line = param.type_identifier.token.position.line;
            let column = param.type_identifier.token.position.column;

            if !scope.get_symbol(&param_type).is_some() {
                return Err(format!(
                    "Type not found: {} at Line {} and at Column {}",
                    param_type, line, column
                ));
            }

            scope.insert_symbol(Symbol::new(
                &param_name,
                SymbolKind::Parameter,
                Some(&param_type),
            ))
        }

        let mut block_scope = Scope::extend(scope);
        self.analyze_block(&function.block, &mut block_scope, block_map)?;

        Ok(())
    }

    fn analyze_statement(
        &self,
        statement: &Statement,
        scope: &mut Scope,
        block_map: &mut HashMap<Uuid, Scope>,
    ) -> Result<(), String> {
        match statement {
            Statement::Block(block) => self.analyze_block(block, scope, block_map),
            Statement::Let(r#let) => self.analyze_let_statement(r#let, scope),
            Statement::Const(r#const) => self.analyze_const_statement(r#const, scope),
            Statement::Assignment(assignment) => {
                self.analyze_assignment_statement(assignment, scope)
            }
            Statement::FunctionCall(call) => self.analyze_function_call(call, scope),
            Statement::While(r#while) => self.analyze_while_statement(r#while, scope, block_map),
            Statement::DoWhile(do_while) => {
                self.analyze_do_while_statement(do_while, scope, block_map)
            }
            Statement::For(r#for) => self.analyze_for_statement(r#for, scope, block_map),
            Statement::If(r#if) => self.analyze_if_statement(r#if, scope, block_map),
            Statement::Break(r#break) => self.analyze_break_statement(r#break, scope),
            Statement::Continue(r#continue) => self.analyze_continue_statement(r#continue, scope),
            Statement::Return(r#return) => self.analyze_return_statement(r#return, scope),
            Statement::Print(print) => self.analyze_print_statement(print, scope),
        }
    }

    fn analyze_return_statement(&self, r#return: &Return, scope: &Scope) -> Result<(), String> {
        match scope.get_fun_return_type() {
            None => Err(format!("Return statement not allowed here")),
            Some(fun_return_type) => {
                let return_value = match &r#return.expression {
                    None => "void".to_string(),
                    Some(e) => self.analyze_expression(&e, scope)?,
                };

                if is_number(&return_value) && is_number(&fun_return_type)
                    || return_value == fun_return_type
                {
                    Ok(())
                } else {
                    Err(format!(
                        "Expected return type '{}', found '{}'",
                        fun_return_type, return_value
                    ))
                }
            }
        }
    }

    fn analyze_break_statement(&self, _: &Break, scope: &Scope) -> Result<(), String> {
        if !scope.is_loop() {
            Err(format!("Break statement outside of loop"))
        } else {
            Ok(())
        }
    }

    fn analyze_continue_statement(&self, _: &Continue, scope: &Scope) -> Result<(), String> {
        if !scope.is_loop() {
            return Err(format!("Continue statement outside of loop"));
        } else {
            Ok(())
        }
    }

    fn analyze_block(
        &self,
        block: &Block,
        scope: &mut Scope,
        block_map: &mut HashMap<Uuid, Scope>,
    ) -> Result<(), String> {
        for statement in &block.statements {
            self.analyze_statement(statement, scope, block_map)?;
        }

        block_map.insert(block.id, scope.clone());

        Ok(())
    }

    fn analyze_let_statement(&self, r#let: &Let, scope: &mut Scope) -> Result<(), String> {
        match r#let {
            Let::WithoutValue(identifier, return_type) => {
                let variable_name = identifier.name.clone();

                let line = identifier.token.position.line;
                let column = identifier.token.position.column;

                if scope.get_symbol(&variable_name).is_some() {
                    return Err(format!(
                        "Duplicated identifier found: {} at Line {} and at Column {}",
                        variable_name, line, column
                    ));
                }

                let return_type_name = return_type.name.clone();

                let line = return_type.token.position.line;
                let column = return_type.token.position.column;

                if !scope.get_symbol(&return_type_name).is_some() {
                    return Err(format!(
                        "Type not found: {} at Line {} and at Column {}",
                        return_type_name, line, column
                    ));
                }

                scope.insert_symbol(Symbol::new(
                    &variable_name,
                    SymbolKind::Variable,
                    Some(&return_type_name),
                ));

                Ok(())
            }
            Let::WithValue(identifier, return_type, expression) => {
                let variable_name = identifier.name.clone();

                let line = identifier.token.position.line;
                let column = identifier.token.position.column;

                if scope.get_symbol(&variable_name).is_some() {
                    return Err(format!(
                        "Duplicated identifier found: {} at Line {} and at Column {}",
                        variable_name, line, column
                    ));
                }

                match return_type {
                    None => {
                        let return_type_name = self.analyze_expression(expression, scope)?;

                        scope.insert_symbol(Symbol::new(
                            &variable_name,
                            SymbolKind::Variable,
                            Some(&return_type_name),
                        ));

                        Ok(())
                    }
                    Some(return_type) => {
                        let return_type_name = return_type.name.clone();

                        let line = return_type.token.position.line;
                        let column = return_type.token.position.column;

                        if !scope.get_symbol(&return_type_name).is_some() {
                            return Err(format!(
                                "Type not found: {} at Line {} and at Column {}",
                                return_type_name, line, column
                            ));
                        }

                        let expression_return_type_name =
                            self.analyze_expression(expression, scope)?;

                        if is_number(&expression_return_type_name) && is_number(&return_type_name)
                            || expression_return_type_name == return_type_name
                        {
                            scope.insert_symbol(Symbol::new(
                                &variable_name,
                                SymbolKind::Variable,
                                Some(&return_type_name),
                            ));

                            Ok(())
                        } else {
                            Err(format!(
                                "Type mismatch, expected: {}, found: {}",
                                return_type_name, expression_return_type_name
                            ))
                        }
                    }
                }
            }
        }
    }

    fn analyze_const_statement(&self, r#const: &Const, scope: &mut Scope) -> Result<(), String> {
        let variable_name = r#const.identifier.name.clone();

        let line = r#const.identifier.token.position.line;
        let column = r#const.identifier.token.position.column;

        if scope.get_symbol(&variable_name).is_some() {
            return Err(format!(
                "Duplicated identifier found: {} at Line {} and at Column {}",
                variable_name, line, column
            ));
        }

        match &r#const.type_identifier {
            None => {
                let return_type_name = self.analyze_expression(&r#const.expression, scope)?;

                scope.insert_symbol(Symbol::new(
                    &variable_name,
                    SymbolKind::Constant,
                    Some(&return_type_name),
                ));

                Ok(())
            }
            Some(return_type) => {
                let return_type_name = return_type.name.clone();

                let line = return_type.token.position.line;
                let column = return_type.token.position.column;

                if !scope.get_symbol(&return_type_name).is_some() {
                    return Err(format!(
                        "Type not found: {} at Line {} and at Column {}",
                        return_type_name, line, column
                    ));
                }

                let expression_return_type_name =
                    self.analyze_expression(&r#const.expression, scope)?;

                if is_number(&expression_return_type_name) && is_number(&return_type_name)
                    || expression_return_type_name == return_type_name
                {
                    scope.insert_symbol(Symbol::new(
                        &variable_name,
                        SymbolKind::Constant,
                        Some(&return_type_name),
                    ));

                    Ok(())
                } else {
                    Err(format!(
                        "Type mismatch, expected: {}, found: {}",
                        return_type_name, expression_return_type_name
                    ))
                }
            }
        }
    }

    fn analyze_assignment_statement(
        &self,
        assignment: &Assignment,
        scope: &Scope,
    ) -> Result<(), String> {
        let identifier_name = assignment.identifier.name.clone();
        let line = assignment.identifier.token.position.line;
        let column = assignment.identifier.token.position.column;

        if !scope.get_symbol(&identifier_name).is_some() {
            return Err(format!(
                "Identifier not found: {} at Line {} and at Column {}",
                identifier_name, line, column
            ));
        }

        let symbol = scope.get_symbol(&identifier_name).unwrap();
        let variable_type = symbol.symbol_type.as_ref().unwrap().clone();
        let expression_return_type = self.analyze_expression(&assignment.expression, scope)?;

        match &symbol.kind {
            SymbolKind::Variable => {
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
                        if !is_number(&variable_type) {
                            return Err(format!(
                                "Type mismatch in {}: expected number for the left-hand side, found {}",
                                assignment.operator.token.value, variable_type
                            ));
                        }

                        if !is_number(&expression_return_type) {
                            return Err(format!(
                                "Type mismatch in {}: expected number for the right-hand side, found {}",
                                assignment.operator.token.value, expression_return_type
                            ));
                        }
                    }
                    _ => return Err(format!("Type mismatch in assignment: {}", identifier_name)),
                };

                if is_number(&variable_type) && is_number(&expression_return_type) {
                    Ok(())
                } else if variable_type == expression_return_type {
                    Ok(())
                } else {
                    Err(format!(
                        "Type mismatch in {}: expected {}, found {}",
                        identifier_name, variable_type, expression_return_type
                    ))
                }
            }
            _ => {
                 Err(format!(
                "Assignment to {} is not allowed; only variables can be reassigned. Found at Line {} and Column {}",
                identifier_name, line, column
            ))
            }
        }
    }

    fn analyze_expression(&self, expression: &Expression, scope: &Scope) -> Result<String, String> {
        match expression {
            Expression::Range(range) => {
                let left_return_type = self.analyze_expression(&range.left, scope)?;
                let right_return_type = self.analyze_expression(&range.right, scope)?;

                match range.operator.token.kind {
                    TokenKind::DotDot | TokenKind::DotDotEquals => {
                        if !is_number(&left_return_type) || !is_number(&right_return_type) {
                            Err(format!(
                                "Expected number for both operands, found {} and {}",
                                left_return_type, right_return_type
                            ))
                        } else {
                            Ok("range".to_string())
                        }
                    }
                    _ => unreachable!(),
                }
            }
            Expression::Identifier(identifier) => {
                let identifier_name = identifier.name.clone();
                let line = identifier.token.position.line;
                let column = identifier.token.position.column;

                if !scope.get_symbol(&identifier_name).is_some() {
                    Err(format!(
                        "Identifier not found: {} at Line {} and at Column {}",
                        identifier_name, line, column
                    ))
                } else {
                    let symbol = scope.get_symbol(&identifier_name).unwrap();

                    match &symbol.kind {
                        SymbolKind::Constant | SymbolKind::Variable | SymbolKind::Parameter => {
                            Ok(symbol.symbol_type.as_ref().unwrap().clone())
                        }
                        _ => Err(format!(
                            "Identifier {} is not a constant, a variable or a parameter at Line {} and Column {}",
                            identifier_name, line, column
                        )),
                    }
                }
            }
            Expression::FunctionCall(function_call) => {
                let identifier_name = function_call.identifier.name.clone();
                let line = function_call.identifier.token.position.line;
                let column = function_call.identifier.token.position.column;

                if !scope.get_symbol(&identifier_name).is_some() {
                    Err(format!(
                        "Identifier not found: {} at Line {} and at Column {}",
                        identifier_name, line, column
                    ))
                } else {
                    let symbol = scope.get_symbol(&identifier_name).unwrap();

                    match &symbol.kind {
                        SymbolKind::Function(_) => {
                            self.analyze_function_call(function_call, scope)?;
                            Ok(symbol.symbol_type.as_ref().unwrap().clone())
                        }
                        _ => Err(format!(
                            "Identifier {} is not a function at Line {} and Column {}",
                            identifier_name, line, column
                        )),
                    }
                }
            }
            Expression::Literal(literal) => match literal {
                Literal::Number(token) => {
                    if token.value.contains(".") {
                        Ok("f32".to_string())
                    } else {
                        Ok("i32".to_string())
                    }
                }
                Literal::Boolean(_) => Ok("bool".to_string()),
                Literal::Char(_) => Ok("char".to_string()),
                Literal::String(_) => Ok("string".to_string()),
            },
            Expression::Parenthesized(parenthesize) => {
                self.analyze_expression(&parenthesize.expression.as_ref(), scope)
            }
            Expression::Unary(unary) => {
                let return_type = self.analyze_expression(&unary.expression, scope)?;

                match unary.operator.token.kind {
                    TokenKind::Tilde => {
                        if !is_integer(&return_type) {
                            Err(format!("Expected integer, found {}", return_type))
                        } else {
                            Ok(return_type)
                        }
                    }
                    TokenKind::Plus | TokenKind::Minus => {
                        if !is_number(&return_type) {
                            Err(format!("Expected type number, found {}", return_type))
                        } else {
                            Ok(return_type)
                        }
                    }
                    TokenKind::Exclamation => {
                        if return_type != "bool" {
                            Err(format!("Expected type 'bool', found {}", return_type))
                        } else {
                            Ok("bool".to_string())
                        }
                    }
                    _ => unreachable!(),
                }
            }
            Expression::Binary(binary) => {
                let left_return_type = self.analyze_expression(&binary.left, scope)?;
                let right_return_type = self.analyze_expression(&binary.right, scope)?;

                match binary.operator.token.kind {
                    TokenKind::EqualsEquals | TokenKind::ExclamationEquals => {
                        if is_number(&left_return_type) && is_number(&right_return_type)
                            || left_return_type == right_return_type
                        {
                            Ok("bool".to_string())
                        } else {
                            Err(format!(
                                "Mismatched types for equality comparison: {} and {}",
                                left_return_type, right_return_type
                            ))
                        }
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
                        if !is_number(&left_return_type) || !is_number(&right_return_type) {
                            Err(format!(
                                "Expected number for both operands, found {} and {}",
                                left_return_type, right_return_type
                            ))
                        } else {
                            let return_type = String::from(number_type_precedence(vec![
                                &left_return_type,
                                &right_return_type,
                            ]));

                            Ok(return_type)
                        }
                    }
                    TokenKind::GreaterThan
                    | TokenKind::GreaterThanEquals
                    | TokenKind::LessThan
                    | TokenKind::LessThanEquals => {
                        if !is_number(&left_return_type) || !is_number(&right_return_type) {
                            Err(format!(
                                "Expected number for both operands, found {} and {}",
                                left_return_type, right_return_type
                            ))
                        } else {
                            Ok("bool".to_string())
                        }
                    }
                    TokenKind::AmpersandAmpersand | TokenKind::PipePipe => {
                        if left_return_type != "bool" || right_return_type != "bool" {
                            Err(format!(
                                "Expected 'bool' for both operands, found {} and {}",
                                left_return_type, right_return_type
                            ))
                        } else {
                            Ok("bool".to_string())
                        }
                    }
                    _ => Err(format!("Mismatch types")),
                }
            }
        }
    }

    fn analyze_function_call(
        &self,
        function_call: &FunctionCall,
        scope: &Scope,
    ) -> Result<(), String> {
        let identifier_name = function_call.identifier.name.clone();
        let line = function_call.identifier.token.position.line;
        let column = function_call.identifier.token.position.column;

        if !scope.get_symbol(&identifier_name).is_some() {
            return Err(format!(
                "Identifier not found: {} at Line {} and at Column {}",
                identifier_name, line, column
            ));
        }

        let symbol = scope.get_symbol(&identifier_name).unwrap();

        match &symbol.kind {
            SymbolKind::Function(params) => {
                if params.len() != function_call.params.expressions.len() {
                    Err(format!(
                        "Expected {} parameters but found {} at Line {} and Column {}",
                        params.len(),
                        function_call.params.expressions.len(),
                        line,
                        column
                    ))
                } else {
                    for i in 0..params.len() {
                        let expected_type_name = params.get(i).unwrap().clone();
                        let found_type_name = self.analyze_expression(
                            function_call.params.expressions.get(i).unwrap(),
                            scope,
                        )?;

                        if is_number(&expected_type_name) && is_number(&found_type_name)
                            || expected_type_name == found_type_name
                        {
                            return Ok(());
                        } else {
                            return  Err(format!(
                                "Expected type {} but found type {} for parameter {} at Line {} and Column {}",
                                expected_type_name,
                                found_type_name,
                                i + 1,
                                line,
                                column
                            ));
                        }
                    }

                    Ok(())
                }
            }
            other => Err(format!(
                "Function expected, found {} at Line {} and at Column {}",
                other, line, column
            )),
        }
    }

    fn analyze_while_statement(
        &self,
        r#while: &While,
        scope: &mut Scope,
        block_map: &mut HashMap<Uuid, Scope>,
    ) -> Result<(), String> {
        let expression_return_type = self.analyze_expression(&r#while.expression, &scope)?;

        if expression_return_type != "bool" {
            Err(format!(
                "Expected 'bool' in while condition, found {}",
                expression_return_type
            ))
        } else {
            let mut block_scope = Scope::block(scope.clone(), true);
            self.analyze_block(&r#while.block, &mut block_scope, block_map)?;

            Ok(())
        }
    }

    fn analyze_do_while_statement(
        &self,
        do_while: &DoWhile,
        scope: &mut Scope,
        block_map: &mut HashMap<Uuid, Scope>,
    ) -> Result<(), String> {
        let mut block_scope = Scope::block(scope.clone(), true);
        self.analyze_block(&do_while.block, &mut block_scope, block_map)?;

        let expression_return_type = self.analyze_expression(&do_while.expression, &scope)?;

        if expression_return_type != "bool" {
            Err(format!(
                "Expected 'bool' in do while condition, found {}",
                expression_return_type
            ))
        } else {
            Ok(())
        }
    }

    fn analyze_if_statement(
        &self,
        r#if: &If,
        scope: &Scope,
        block_map: &mut HashMap<Uuid, Scope>,
    ) -> Result<(), String> {
        let expression_return_type = self.analyze_expression(&r#if.expression, scope)?;

        if expression_return_type != "bool" {
            Err(format!(
                "Expected 'bool' in if condition, found {}",
                expression_return_type
            ))
        } else {
            let mut block_scope = Scope::block(scope.clone(), false);
            self.analyze_block(&r#if.block, &mut block_scope, block_map)?;

            if let Some(r#else) = &r#if.r#else {
                let mut block_scope = Scope::block(scope.clone(), false);
                self.analyze_block(&r#else.block, &mut block_scope, block_map)?;
            }

            Ok(())
        }
    }

    fn analyze_for_statement(
        &self,
        r#for: &For,
        scope: &Scope,
        block_map: &mut HashMap<Uuid, Scope>,
    ) -> Result<(), String> {
        let mut block_scope = Scope::block(scope.clone(), true);

        let identifier_name = r#for.identifier.name.clone();
        let line = r#for.identifier.token.position.line;
        let column = r#for.identifier.token.position.column;

        if block_scope.get_symbol(&identifier_name).is_some() {
            Err(format!(
                "Duplicated identifier found: {} at Line {} and at Column {}",
                identifier_name, line, column
            ))
        } else {
            block_scope.insert_symbol(Symbol::new(
                &identifier_name,
                SymbolKind::Constant,
                Some("i32"),
            ));

            match &r#for.expression {
                Expression::Range(_) => {
                    self.analyze_expression(&r#for.expression, &block_scope)?;
                    self.analyze_block(&r#for.block, &mut block_scope, block_map)?;

                    Ok(())
                }
                expression => Err(format!("Expected range expression found {}", expression)),
            }
        }
    }

    fn analyze_print_statement(&self, print: &Print, scope: &Scope) -> Result<(), String> {
        for expression in &print.expressions {
            self.analyze_expression(expression, scope)?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::Analyzer;

    #[test]
    fn test_expression() {
        let code = "
            fun main() {
                let x = 2;
            }
        ";

        let mut analyzer = Analyzer::from_code(code).unwrap();
        let result = analyzer.analyze();

        assert!(result.is_ok());
    }
}
