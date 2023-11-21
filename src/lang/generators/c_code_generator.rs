use crate::lang::syntax::parser::{
    compilation_unit::CompilationUnit,
    expressions::{expression::Expression, literal::Literal},
    shared::{block::Block, function_call::FunctionCall},
    statements::{r#let::Let, r#return::Return, statement::Statement},
    top_level_statements::top_level_statement::TopLevelStatement,
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
        _ => unreachable!(),
    }
    .to_string()
}

pub struct CCodeGenerator {
    ast: CompilationUnit,
}

impl CCodeGenerator {
    pub fn from_ast(ast: CompilationUnit) -> Self {
        Self { ast }
    }

    pub fn generate(&self) -> String {
        let mut code = String::new();

        for statement in &self.ast.statements {
            self.generate_top_level_statement(statement, &mut code);
        }

        return code.clone();
    }

    fn generate_top_level_statement(&self, statement: &TopLevelStatement, code: &mut String) {
        match statement {
            TopLevelStatement::Function(function) => {
                let return_type_name = function
                    .type_identifier
                    .as_ref()
                    .map_or("void".to_string(), |id| id.name.clone());

                let function_name = function.identifier.name.clone();

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
        code.push_str("{");

        for statement in &block.statements {
            self.generate_statement(statement, code);
        }

        code.push_str("}");
    }

    fn generate_statement(&self, statement: &Statement, code: &mut String) {
        match statement {
            Statement::Let(r#let) => self.generate_let_statement(r#let, code),
            Statement::FunctionCall(call) => self.generate_function_call_statement(call, code),
            Statement::Return(r#return) => self.generate_return_statement(r#return, code),
            _ => {}
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

    fn generate_let_statement(&self, r#let: &Let, code: &mut String) {
        match r#let {
            Let::WithValue(identifier, type_identifier, expression) => {
                let identifier_name = identifier.name.clone();

                let type_identifier_name: String;

                // code.push_str(&format!(
                //     "{} {}=",
                //     convert_to_c_type(&type_identifier_name),
                //     identifier_name
                // ));

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
                Literal::Number(token) | Literal::Char(token) => code.push_str(&token.value),
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
}
