use std::collections::HashMap;

use uuid::Uuid;

use crate::lang::{
    sematic::{expression_analyzer::ExpressionAnalyzer, scope::Scope},
    syntax::{
        lexer::token_kind::TokenKind,
        parser::{
            compilation_unit::CompilationUnit,
            expressions::{expression::Expression, literal::Literal},
            shared::{block::Block, function_call::FunctionCall},
            statements::{
                assignment::Assignment, do_while::DoWhile, print::Print, r#break::Break,
                r#const::Const, r#continue::Continue, r#for::For, r#if::If, r#let::Let,
                r#return::Return, r#while::While, statement::Statement,
            },
            top_level_statements::top_level_statement::TopLevelStatement,
        },
    },
};

fn convert_to_c_type(lang_type: &str) -> String {
    match lang_type {
        "void" => "void",
        "char" => "unsigned char",
        "bool" => "unsigned char",
        "u8" => "unsigned char",
        "i8" => "signed char",
        "u16" => "unsigned short int",
        "i16" => "signed short int",
        "u32" => "unsigned int",
        "i32" => "signed int",
        "u64" => "unsigned long long int",
        "i64" => "signed long long int",
        "f32" => "float",
        "f64" => "double",
        "string" => "char*",
        _ => unreachable!(),
    }
    .to_string()
}

pub struct CCodeGenerator {
    ast: CompilationUnit,
    block_map: HashMap<Uuid, Scope>,
}

impl CCodeGenerator {
    pub fn from_ast(ast: CompilationUnit, block_map: HashMap<Uuid, Scope>) -> Self {
        Self { ast, block_map }
    }

    pub fn generate(&self) -> String {
        let mut code = String::from("#include <stdio.h>\n");

        for statement in &self.ast.statements {
            self.generate_top_level_statement(statement, &mut code);
        }

        return code.clone();
    }

    fn generate_top_level_statement(&self, statement: &TopLevelStatement, code: &mut String) {
        match statement {
            TopLevelStatement::Function(function) => {
                // TODO: Deal with the main function when converting to C.

                let function_name = function.identifier.name.clone();

                let return_type_name = function
                    .type_identifier
                    .as_ref()
                    .map_or("void".to_string(), |id| id.name.clone());

                code.push_str(&format!(
                    "{} {}(",
                    convert_to_c_type(&return_type_name),
                    function_name
                ));

                for (index, param) in function.params_declaration.params.iter().enumerate() {
                    code.push_str(&format!(
                        "{} {}",
                        convert_to_c_type(&param.type_identifier.name),
                        param.identifier.name
                    ));

                    if index != function.params_declaration.params.len() - 1 {
                        code.push_str(",");
                    }
                }

                code.push_str(")");

                self.generate_block_statement(&function.block, code);
            }
        }
    }

    fn generate_block_statement(&self, block: &Block, code: &mut String) {
        let scope = self.block_map.get(&block.id).unwrap();

        code.push_str("{");

        for statement in &block.statements {
            self.generate_statement(statement, scope, code);
        }

        code.push_str("}");
    }

    fn generate_statement(&self, statement: &Statement, scope: &Scope, code: &mut String) {
        match statement {
            Statement::Block(block) => self.generate_block_statement(block, code),
            Statement::Let(r#let) => self.generate_let_statement(r#let, scope, code),
            Statement::Const(r#const) => self.generate_const_statement(r#const, scope, code),
            Statement::FunctionCall(call) => self.generate_function_call_statement(call, code),
            Statement::Return(r#return) => self.generate_return_statement(r#return, code),
            Statement::Assignment(assignment) => {
                self.generate_assignment_statement(assignment, code)
            }
            Statement::If(r#if) => self.generate_if_statement(r#if, code),
            Statement::While(r#while) => self.generate_while_statement(r#while, code),
            Statement::DoWhile(do_while) => self.generate_do_while_statement(do_while, code),
            Statement::For(r#for) => self.generate_for_statement(r#for, code),
            Statement::Break(r#break) => self.generate_break_statement(r#break, code),
            Statement::Continue(r#continue) => self.generate_continue_statement(r#continue, code),
            Statement::Print(print) => self.generate_print_statement(print, scope, code),
        }
    }

    fn generate_return_statement(&self, r#return: &Return, code: &mut String) {
        match &r#return.expression {
            None => code.push_str("return;"),
            Some(expression) => {
                code.push_str("return ");
                self.generate_expression(expression, code);
                code.push_str(";");
            }
        }
    }

    fn generate_let_statement(&self, r#let: &Let, scope: &Scope, code: &mut String) {
        match r#let {
            Let::WithValue(identifier, _, expression) => {
                let identifier_name = identifier.name.clone();

                let type_identifier_name = scope
                    .get_symbol(&identifier_name)
                    .unwrap()
                    .symbol_type
                    .as_ref()
                    .unwrap();

                code.push_str(&format!(
                    "{} {}=",
                    convert_to_c_type(&type_identifier_name),
                    identifier_name
                ));

                self.generate_expression(expression, code);

                code.push_str(";")
            }
            Let::WithoutValue(identifier, type_identifier) => {
                let identifier_name = identifier.name.clone();
                let type_identifier_name = type_identifier.name.clone();

                code.push_str(&format!(
                    "{} {};",
                    convert_to_c_type(&type_identifier_name),
                    identifier_name
                ));
            }
        }
    }

    fn generate_const_statement(&self, r#const: &Const, scope: &Scope, code: &mut String) {
        let identifier_name = r#const.identifier.name.clone();

        let type_identifier_name = scope
            .get_symbol(&identifier_name)
            .unwrap()
            .symbol_type
            .as_ref()
            .unwrap();

        code.push_str(&format!(
            "const {} {}=",
            convert_to_c_type(type_identifier_name),
            identifier_name
        ));

        self.generate_expression(&r#const.expression, code);
        code.push_str(";");
    }

    fn generate_expression(&self, expression: &Expression, code: &mut String) {
        match expression {
            Expression::Identifier(identifier) => {
                code.push_str(&identifier.name);
            }
            Expression::FunctionCall(call) => {
                code.push_str(&call.identifier.name);
                code.push_str("(");

                for (index, expression) in call.params.expressions.iter().enumerate() {
                    self.generate_expression(expression, code);
                    if index != call.params.expressions.len() - 1 {
                        code.push_str(",");
                    }
                }

                code.push_str(")");
            }
            Expression::Unary(unary) => {
                code.push_str(&unary.operator.token.value);
                self.generate_expression(&unary.expression, code);
            }
            Expression::Literal(literal) => match literal {
                Literal::Number(token) => code.push_str(&token.value),
                Literal::Char(token) => code.push_str(&format!("'{}'", token.value)),
                Literal::String(token) => code.push_str(&format!("\"{}\"", token.value)),
                Literal::Boolean(token) => match &token.value[..] {
                    "true" => code.push_str("1"),
                    _ => code.push_str("0"),
                },
            },
            Expression::Binary(binary) => {
                self.generate_expression(&binary.left, code);
                code.push_str(&binary.operator.token.value);
                self.generate_expression(&binary.right, code);
            }
            Expression::Parenthesized(parenthesized) => {
                code.push_str("(");
                self.generate_expression(&parenthesized.expression, code);
                code.push_str(")");
            }
            _ => {}
        }
    }

    pub fn generate_function_call_statement(&self, call: &FunctionCall, code: &mut String) {
        code.push_str(&call.identifier.name);
        code.push_str("(");

        for (index, expression) in call.params.expressions.iter().enumerate() {
            self.generate_expression(expression, code);
            if index != call.params.expressions.len() - 1 {
                code.push_str(",");
            }
        }

        code.push_str(");");
    }

    pub fn generate_assignment_statement(&self, assignment: &Assignment, code: &mut String) {
        code.push_str(&assignment.identifier.name);
        code.push_str(&assignment.operator.name);

        self.generate_expression(&assignment.expression, code);

        code.push_str(";");
    }

    pub fn generate_if_statement(&self, r#if: &If, code: &mut String) {
        code.push_str("if(");
        self.generate_expression(&r#if.expression, code);
        code.push_str(")");
        self.generate_block_statement(&r#if.block, code);

        match &r#if.r#else {
            None => {}
            Some(r#else) => {
                code.push_str("else");
                self.generate_block_statement(&r#else.block, code);
            }
        }
    }

    pub fn generate_while_statement(&self, r#while: &While, code: &mut String) {
        code.push_str("while(");
        self.generate_expression(&r#while.expression, code);
        code.push_str(")");
        self.generate_block_statement(&r#while.block, code);
    }

    pub fn generate_do_while_statement(&self, do_while: &DoWhile, code: &mut String) {
        code.push_str("do");
        self.generate_block_statement(&do_while.block, code);
        code.push_str("while (");
        self.generate_expression(&do_while.expression, code);
        code.push_str(");");
    }

    pub fn generate_for_statement(&self, r#for: &For, code: &mut String) {
        if let Expression::Range(range) = &r#for.expression {
            code.push_str("int ");
            code.push_str(&format!("{};", r#for.identifier.name));
            code.push_str("for(");
            code.push_str(&format!("{}=", r#for.identifier.name));
            self.generate_expression(&range.left, code);
            code.push_str(";");

            match range.operator.token.kind {
                TokenKind::DotDot => {
                    code.push_str(&format!("{}<", r#for.identifier.name));
                    self.generate_expression(&range.right, code);
                }
                TokenKind::DotDotEquals => {
                    code.push_str(&format!("{}<=", r#for.identifier.name));
                    self.generate_expression(&range.right, code);
                }
                _ => unreachable!(),
            }

            code.push_str(";");
            code.push_str(&format!("{}++)", r#for.identifier.name));
            self.generate_block_statement(&r#for.block, code);
        }
    }

    fn generate_break_statement(&self, _: &Break, code: &mut String) {
        code.push_str("break;");
    }

    fn generate_continue_statement(&self, _: &Continue, code: &mut String) {
        code.push_str("continue;");
    }

    fn generate_print_statement(&self, print: &Print, scope: &Scope, code: &mut String) {
        for expression in &print.expressions {
            code.push_str("printf(\"");

            let mut analyzer = ExpressionAnalyzer::new(expression.clone(), scope.clone());

            let expression_return_type = &analyzer.analyze().unwrap()[..];

            let c_print_shortcut = match expression_return_type {
                "string" => "%s",
                "i32" => "%d",
                "i64" => "%ld",
                "u32" => "%u",
                "u64" => "%lu",
                "f32" => "%f",
                "f64" => "%lf",
                "bool" => "%c",
                "char" => "%c",
                _ => "",
            };

            code.push_str(&format!("{}", c_print_shortcut));
            code.push_str("\",");
            self.generate_expression(expression, code);
            code.push_str(");");
        }

        if print.new_line {
            code.push_str("printf(\"\\n\");");
        }
    }
}
