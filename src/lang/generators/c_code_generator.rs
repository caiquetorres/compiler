use std::{cell::RefCell, rc::Rc};

use crate::lang::{
    lexer::token_kind::TokenKind,
    semantic::{
        analyzer::Scopes, expressions::expression_analyzer::ExpressionAnalyzer, scope::Scope,
        semantic_type::SemanticType, symbol::Symbol,
    },
    syntax::parser::{
        compilation_unit::CompilationUnit,
        expressions::{
            expression::{Expression, ExpressionMeta},
            literal::Literal,
        },
        shared::block::Block,
        statements::{
            assignment::Assignment, do_while::DoWhile, print::Print, r#break::Break,
            r#continue::Continue, r#for::For, r#if::If, r#let::Let, r#return::Return,
            r#while::While, statement::Statement,
        },
        top_level_statements::{function::Function, top_level_statement::TopLevelStatement},
    },
};

fn convert_to_c_type(lang_type: SemanticType) -> String {
    match lang_type {
        SemanticType::Void => "void",
        SemanticType::Char => "unsigned char",
        SemanticType::Bool => "unsigned char",
        SemanticType::U8 => "unsigned char",
        SemanticType::I8 => "signed char",
        SemanticType::U16 => "unsigned short int",
        SemanticType::I16 => "signed short int",
        SemanticType::U32 => "unsigned int",
        SemanticType::I32 => "signed int",
        SemanticType::U64 => "unsigned long long int",
        SemanticType::I64 => "signed long long int",
        SemanticType::F32 => "float",
        SemanticType::F64 => "double",
        SemanticType::String => "char*",
        SemanticType::Any => "void *",
        SemanticType::Array(array_type, _) => {
            let mut root_type = array_type.as_ref().clone();

            loop {
                if let SemanticType::Array(array_type, _) = &root_type {
                    root_type = array_type.as_ref().clone();
                } else {
                    break;
                }
            }

            let result = convert_to_c_type(root_type.clone());
            return result;
        }
        _ => unreachable!(),
    }
    .to_string()
}

pub struct CCodeGenerator<'s, 'a> {
    scopes: &'s Scopes,
    ast: &'a CompilationUnit,
}

impl<'s, 'a> CCodeGenerator<'s, 'a> {
    pub fn new(ast: &'a CompilationUnit, scopes: &'s Scopes) -> Self {
        Self { ast, scopes }
    }

    pub fn generate(&self) -> String {
        let mut code = String::from("#include <stdio.h>\n");

        for statement in &self.ast.statements {
            match statement {
                TopLevelStatement::Function(function) => {
                    self.generate_prototypes(function, &mut code)
                }
            }
        }

        for statement in &self.ast.statements {
            match statement {
                TopLevelStatement::Function(function) => {
                    self.generate_top_level_statement(function, &mut code)
                }
            }
        }

        code
    }

    fn generate_prototypes(&self, function: &Function, code: &mut String) {
        let function_name = function.identifier.name.clone();
        let is_main = function_name == "main";

        if is_main {
            return;
        }

        let function_return_type = if is_main {
            SemanticType::I32
        } else {
            function
                .r#type
                .as_ref()
                .map_or(SemanticType::Void, |id| SemanticType::from_type(id.clone()))
        };

        code.push_str(&format!(
            "{} {}(",
            convert_to_c_type(function_return_type),
            function_name
        ));

        for (index, param) in function.params_declaration.params.iter().enumerate() {
            let r#type = SemanticType::from_type(param.r#type.clone());
            code.push_str(&format!(
                "{} {}",
                convert_to_c_type(r#type.clone()),
                param.identifier.name
            ));

            self.generate_array_description(&r#type, code);

            if index != function.params_declaration.params.len() - 1 {
                code.push_str(",");
            }
        }

        code.push_str(");");
    }

    fn generate_top_level_statement(&self, function: &Function, code: &mut String) {
        let function_name = function.identifier.name.clone();

        let is_main = function_name == "main";

        let function_return_type = if is_main {
            SemanticType::I32
        } else {
            function
                .r#type
                .as_ref()
                .map_or(SemanticType::Void, |id| SemanticType::from_type(id.clone()))
        };

        code.push_str(&format!(
            "{} {}(",
            convert_to_c_type(function_return_type),
            function_name
        ));

        for (index, param) in function.params_declaration.params.iter().enumerate() {
            let r#type = SemanticType::from_type(param.r#type.clone());
            code.push_str(&format!(
                "{} {}",
                convert_to_c_type(r#type.clone()),
                param.identifier.name
            ));

            self.generate_array_description(&r#type, code);

            if index != function.params_declaration.params.len() - 1 {
                code.push_str(",");
            }
        }

        code.push_str(")");

        self.generate_block_statement(&function.block, code);

        if is_main {
            code.pop();
            code.push_str("return 0;}");
        }
    }

    fn generate_block_statement(&self, block: &Block, code: &mut String) {
        let scope = self.scopes.get(&block.id).unwrap().clone();

        code.push_str("{");

        for statement in &block.statements {
            self.generate_statement(statement, Rc::clone(&scope), code);
        }

        code.push_str("}");
    }

    fn generate_statement(
        &self,
        statement: &Statement,
        scope: Rc<RefCell<Scope>>,
        code: &mut String,
    ) {
        match statement {
            Statement::Expression(expression) => {
                self.generate_expression(expression, Rc::clone(&scope), code);
                code.push_str(";");
            }
            Statement::Block(block) => self.generate_block_statement(block, code),
            Statement::Let(r#let) => self.generate_let_statement(r#let, Rc::clone(&scope), code),
            Statement::Return(r#return) => {
                self.generate_return_statement(r#return, Rc::clone(&scope), code)
            }
            Statement::Assignment(assignment) => {
                self.generate_assignment_statement(assignment, Rc::clone(&scope), code)
            }
            Statement::If(r#if) => self.generate_if_statement(r#if, Rc::clone(&scope), code),
            Statement::While(r#while) => {
                self.generate_while_statement(r#while, Rc::clone(&scope), code)
            }
            Statement::DoWhile(do_while) => {
                self.generate_do_while_statement(do_while, Rc::clone(&scope), code)
            }
            Statement::For(r#for) => self.generate_for_statement(r#for, code),
            Statement::Break(r#break) => self.generate_break_statement(r#break, code),
            Statement::Continue(r#continue) => self.generate_continue_statement(r#continue, code),
            Statement::Print(print) => {
                self.generate_print_statement(print, Rc::clone(&scope), code)
            }
        }
    }

    fn generate_return_statement(
        &self,
        r#return: &Return,
        scope: Rc<RefCell<Scope>>,
        code: &mut String,
    ) {
        if scope.borrow().get_function_name().unwrap() == "main" {
            code.push_str("return 0;");
        } else {
            match &r#return.expression {
                None => code.push_str("return;"),
                Some(expression) => {
                    code.push_str("return ");
                    self.generate_expression(expression, Rc::clone(&scope), code);
                    code.push_str(";");
                }
            }
        }
    }

    fn generate_array_description(&self, r#type: &SemanticType, code: &mut String) {
        if let SemanticType::Array(array_type, size) = r#type {
            code.push_str(&format!("[{}]", size));
            self.generate_array_description(&array_type, code);
        }
    }

    fn generate_let_statement(&self, r#let: &Let, scope: Rc<RefCell<Scope>>, code: &mut String) {
        let identifier_name = r#let.identifier.name.clone();
        let type_identifier = scope.borrow().get(&identifier_name).unwrap();

        if let Symbol::Variable { symbol_type, .. } = type_identifier {
            code.push_str(&format!(
                "{} {}",
                convert_to_c_type(symbol_type.clone()),
                identifier_name
            ));

            self.generate_array_description(&symbol_type, code);
        }

        if let Some(expression) = &r#let.expression {
            code.push_str("=");
            self.generate_expression(expression, Rc::clone(&scope), code);
        }

        code.push_str(";")
    }

    fn generate_meta(&self, meta: &ExpressionMeta, scope: Rc<RefCell<Scope>>, code: &mut String) {
        match meta {
            ExpressionMeta::Call(expressions, meta) => {
                code.push_str("(");

                for (index, expression) in expressions.iter().enumerate() {
                    self.generate_expression(expression, Rc::clone(&scope), code);

                    if index != expressions.len() - 1 {
                        code.push_str(",");
                    }
                }

                code.push_str(")");

                if let Some(meta) = meta.as_ref() {
                    self.generate_meta(meta, Rc::clone(&scope), code);
                }
            }
            ExpressionMeta::Index(expression, meta) => {
                code.push_str("[");
                self.generate_expression(expression, Rc::clone(&scope), code);
                code.push_str("]");

                if let Some(meta) = meta.as_ref() {
                    self.generate_meta(meta, Rc::clone(&scope), code);
                }
            }
        }
    }

    fn generate_expression(
        &self,
        expression: &Expression,
        scope: Rc<RefCell<Scope>>,
        code: &mut String,
    ) {
        match expression {
            Expression::Identifier(identifier, meta) => {
                code.push_str(&identifier.name);

                if let Some(meta) = &meta {
                    self.generate_meta(meta, Rc::clone(&scope), code);
                }
            }
            Expression::Array(array, meta) => {
                code.push_str("{");

                for expression in &array.expressions {
                    self.generate_expression(expression, Rc::clone(&scope), code);
                    code.push_str(",");
                }

                code.push_str("}");

                if let Some(meta) = &meta {
                    self.generate_meta(meta, Rc::clone(&scope), code);
                }
            }
            Expression::Unary(unary) => {
                code.push_str(&unary.operator.token.value);
                self.generate_expression(&unary.expression, Rc::clone(&scope), code);
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
                self.generate_expression(&binary.left, Rc::clone(&scope), code);
                code.push_str(&binary.operator.token.value);
                self.generate_expression(&binary.right, Rc::clone(&scope), code);
            }
            Expression::Parenthesized(parenthesized, meta) => {
                code.push_str("(");
                self.generate_expression(&parenthesized.expression, Rc::clone(&scope), code);
                code.push_str(")");

                if let Some(meta) = meta.as_ref() {
                    self.generate_meta(meta, Rc::clone(&scope), code);
                }
            }
            _ => {}
        }
    }

    pub fn generate_assignment_statement(
        &self,
        assignment: &Assignment,
        scope: Rc<RefCell<Scope>>,
        code: &mut String,
    ) {
        self.generate_expression(&assignment.left, Rc::clone(&scope), code);
        code.push_str(&assignment.operator.name);
        self.generate_expression(&assignment.right, Rc::clone(&scope), code);

        code.push_str(";");
    }

    pub fn generate_if_statement(&self, r#if: &If, scope: Rc<RefCell<Scope>>, code: &mut String) {
        code.push_str("if(");
        self.generate_expression(&r#if.expression, Rc::clone(&scope), code);
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

    fn generate_while_statement(
        &self,
        r#while: &While,
        scope: Rc<RefCell<Scope>>,
        code: &mut String,
    ) {
        code.push_str("while(");
        self.generate_expression(&r#while.expression, Rc::clone(&scope), code);
        code.push_str(")");
        self.generate_block_statement(&r#while.block, code);
    }

    fn generate_do_while_statement(
        &self,
        do_while: &DoWhile,
        scope: Rc<RefCell<Scope>>,
        code: &mut String,
    ) {
        code.push_str("do");
        self.generate_block_statement(&do_while.block, code);
        code.push_str("while(");
        self.generate_expression(&do_while.expression, Rc::clone(&scope), code);
        code.push_str(");");
    }

    fn generate_for_statement(&self, r#for: &For, code: &mut String) {
        let scope = self.scopes.get(&r#for.block.id).unwrap().clone();

        if let Expression::Range(range) = &r#for.expression {
            let symbol = scope.borrow().get(&r#for.identifier.name).unwrap();

            code.push_str("for(");

            match &symbol {
                Symbol::Variable { symbol_type, .. } => {
                    code.push_str(&format!("{} ", convert_to_c_type(symbol_type.clone())));
                }
                _ => unreachable!(),
            }

            code.push_str(&format!("{}=", r#for.identifier.name));
            self.generate_expression(&range.left, Rc::clone(&scope), code);
            code.push_str(";");

            match range.operator.token.kind {
                TokenKind::DotDot => {
                    code.push_str(&format!("{}<", r#for.identifier.name));
                    self.generate_expression(&range.right, Rc::clone(&scope), code);
                }
                TokenKind::DotDotEquals => {
                    code.push_str(&format!("{}<=", r#for.identifier.name));
                    self.generate_expression(&range.right, Rc::clone(&scope), code);
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

    fn generate_print_statement(
        &self,
        print: &Print,
        scope: Rc<RefCell<Scope>>,
        code: &mut String,
    ) {
        for expression in &print.expressions {
            code.push_str("printf(\"");

            let analyzer = ExpressionAnalyzer::analyze(expression, Rc::clone(&scope));
            let return_type = analyzer.return_type;

            let c_print_shortcut = match &return_type {
                SemanticType::String => "%s",
                SemanticType::I8 => "%d",
                SemanticType::U8 => "%u",
                SemanticType::I16 => "%d",
                SemanticType::U16 => "%u",
                SemanticType::I32 => "%d",
                SemanticType::U32 => "%u",
                SemanticType::I64 => "%lld",
                SemanticType::U64 => "%llu",
                SemanticType::F32 => "%ff",
                SemanticType::F64 => "%lf",
                SemanticType::Bool => "%s",
                SemanticType::Char => "%c",
                SemanticType::Ref(_) => "%p",
                _ => "",
            };

            code.push_str(&format!("{}", c_print_shortcut));
            code.push_str("\",");
            self.generate_expression(expression, Rc::clone(&scope), code);

            if return_type.is_bool() {
                code.push_str("?\"true\":\"false\"");
            }

            code.push_str(");");
        }

        if print.new_line {
            code.push_str("printf(\"\\n\");");
        }
    }
}
