use std::{cell::RefCell, rc::Rc};

use crate::lang::{
    semantic::{
        analyzer::Scopes, expression_analyzer::ExpressionAnalyzer, lang_type::LangType,
        scope::Scope, symbol::Symbol,
    },
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
            top_level_statements::{function::Function, top_level_statement::TopLevelStatement},
        },
    },
};

fn convert_to_c_type(lang_type: LangType) -> String {
    match lang_type {
        LangType::Void => "void",
        LangType::Char => "unsigned char",
        LangType::Bool => "unsigned char",
        LangType::U8 => "unsigned char",
        LangType::I8 => "signed char",
        LangType::U16 => "unsigned short int",
        LangType::I16 => "signed short int",
        LangType::U32 => "unsigned int",
        LangType::I32 => "signed int",
        LangType::U64 => "unsigned long long int",
        LangType::I64 => "signed long long int",
        LangType::F32 => "float",
        LangType::F64 => "double",
        LangType::String => "char*",
        LangType::Array(lang_type, ..) => {
            let array_type = convert_to_c_type(lang_type.as_ref().clone());
            let s = format!("{}*", array_type);
            return s;
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
            LangType::I32
        } else {
            function
                .r#type
                .as_ref()
                .map_or(LangType::Void, |id| LangType::from_type(id.clone()))
        };

        code.push_str(&format!(
            "{} {}(",
            convert_to_c_type(function_return_type),
            function_name
        ));

        for (index, param) in function.params_declaration.params.iter().enumerate() {
            code.push_str(&format!(
                "{}",
                convert_to_c_type(LangType::from_type(param.r#type.clone())),
            ));

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
            LangType::I32
        } else {
            function
                .r#type
                .as_ref()
                .map_or(LangType::Void, |id| LangType::from_type(id.clone()))
        };

        code.push_str(&format!(
            "{} {}(",
            convert_to_c_type(function_return_type),
            function_name
        ));

        for (index, param) in function.params_declaration.params.iter().enumerate() {
            code.push_str(&format!(
                "{} {}",
                convert_to_c_type(LangType::from_type(param.r#type.clone())),
                param.identifier.name
            ));

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
            Statement::Block(block) => self.generate_block_statement(block, code),
            Statement::Let(r#let) => self.generate_let_statement(r#let, Rc::clone(&scope), code),
            Statement::Const(r#const) => {
                self.generate_const_statement(r#const, Rc::clone(&scope), code)
            }
            Statement::FunctionCall(call) => {
                self.generate_function_call_statement(call, Rc::clone(&scope), code)
            }
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
            Statement::For(r#for) => self.generate_for_statement(r#for, Rc::clone(&scope), code),
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

    fn generate_let_statement(&self, r#let: &Let, scope: Rc<RefCell<Scope>>, code: &mut String) {
        let identifier_name = r#let.identifier.name.clone();
        let type_identifier = scope.borrow().get(&identifier_name).unwrap();

        match type_identifier {
            Symbol::Variable { symbol_type, .. } => {
                code.push_str(&format!(
                    "{} {}",
                    convert_to_c_type(symbol_type),
                    identifier_name
                ));
            }
            _ => unreachable!(),
        }

        if let Some(expression) = &r#let.expression {
            code.push_str("=");
            self.generate_expression(expression, Rc::clone(&scope), code);
        }

        code.push_str(";")
    }

    fn generate_const_statement(
        &self,
        r#const: &Const,
        scope: Rc<RefCell<Scope>>,
        code: &mut String,
    ) {
        let identifier_name = r#const.identifier.name.clone();

        let type_identifier_name = match scope.borrow().get(&identifier_name).unwrap() {
            Symbol::Const { symbol_type, .. } => symbol_type,
            _ => unreachable!(),
        };

        code.push_str(&format!(
            "const {} {}=",
            convert_to_c_type(type_identifier_name),
            identifier_name
        ));

        self.generate_expression(&r#const.expression, Rc::clone(&scope), code);
        code.push_str(";");
    }

    fn generate_expression(
        &self,
        expression: &Expression,
        scope: Rc<RefCell<Scope>>,
        code: &mut String,
    ) {
        match expression {
            Expression::Identifier(identifier) => {
                code.push_str(&identifier.name);
            }
            Expression::Array(array) => {
                code.push_str("(");

                if array.expressions.len() != 0 {
                    let first_array_expression = array.expressions.get(0).unwrap();

                    let analyzer =
                        ExpressionAnalyzer::analyze(first_array_expression, Rc::clone(&scope));

                    code.push_str(&convert_to_c_type(LangType::from(analyzer.return_type)));
                } else {
                    code.push_str(&convert_to_c_type(LangType::Void));
                }

                code.push_str(&format!("[{}]", array.expressions.len()));
                code.push_str(")");

                code.push_str("{");

                for expression in &array.expressions {
                    self.generate_expression(expression, Rc::clone(&scope), code);
                    code.push_str(",");
                }

                code.push_str("}");
            }
            Expression::FunctionCall(call) => {
                code.push_str(&call.identifier.name);
                code.push_str("(");

                for (index, expression) in call.params.expressions.iter().enumerate() {
                    self.generate_expression(expression, Rc::clone(&scope), code);
                    if index != call.params.expressions.len() - 1 {
                        code.push_str(",");
                    }
                }

                code.push_str(")");
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
            Expression::Parenthesized(parenthesized) => {
                code.push_str("(");
                self.generate_expression(&parenthesized.expression, Rc::clone(&scope), code);
                code.push_str(")");
            }
            _ => {}
        }
    }

    pub fn generate_function_call_statement(
        &self,
        call: &FunctionCall,
        scope: Rc<RefCell<Scope>>,
        code: &mut String,
    ) {
        code.push_str(&call.identifier.name);
        code.push_str("(");

        for (index, expression) in call.params.expressions.iter().enumerate() {
            self.generate_expression(expression, Rc::clone(&scope), code);
            if index != call.params.expressions.len() - 1 {
                code.push_str(",");
            }
        }

        code.push_str(");");
    }

    pub fn generate_assignment_statement(
        &self,
        assignment: &Assignment,
        scope: Rc<RefCell<Scope>>,
        code: &mut String,
    ) {
        code.push_str(&assignment.identifier.name);
        code.push_str(&assignment.operator.name);

        self.generate_expression(&assignment.expression, Rc::clone(&scope), code);

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
        code.push_str("while (");
        self.generate_expression(&do_while.expression, Rc::clone(&scope), code);
        code.push_str(");");
    }

    fn generate_for_statement(&self, r#for: &For, scope: Rc<RefCell<Scope>>, code: &mut String) {
        if let Expression::Range(range) = &r#for.expression {
            code.push_str("int ");
            code.push_str(&format!("{};", r#for.identifier.name));
            code.push_str("for(");
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

            let c_print_shortcut = match return_type.to_string().as_str() {
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
            self.generate_expression(expression, Rc::clone(&scope), code);
            code.push_str(");");
        }

        if print.new_line {
            code.push_str("printf(\"\\n\");");
        }
    }
}
