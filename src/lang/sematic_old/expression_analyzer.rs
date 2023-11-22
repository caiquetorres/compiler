use crate::lang::syntax::{
    lexer::token_kind::TokenKind,
    parser::{
        expressions::{
            binary::Binary, expression::Expression, literal::Literal, parenthesized::Parenthesized,
            range::Range, unary::Unary,
        },
        shared::{function_call::FunctionCall, identifier::Identifier},
    },
};

use super::{scope::Scope, symbol::SymbolKind};

type LangType = String;

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

pub struct ExpressionAnalyzer {
    scope: Scope,
    expression: Expression,
}

impl ExpressionAnalyzer {
    pub fn new(expression: Expression, scope: Scope) -> Self {
        Self { scope, expression }
    }

    pub fn analyze(&mut self) -> Result<LangType, String> {
        self.analyze_expression(&self.expression)
    }

    fn analyze_expression(&self, expression: &Expression) -> Result<LangType, String> {
        match expression {
            Expression::Identifier(identifier) => self.analyze_identifier_expression(&identifier),
            Expression::FunctionCall(call) => self.analyze_function_call_expression(call),
            Expression::Literal(literal) => self.analyze_literal_expression(literal),
            Expression::Unary(unary) => self.analyze_unary_expression(unary),
            Expression::Parenthesized(parenthesized) => {
                self.analyze_parenthesized_expression(parenthesized)
            }
            Expression::Binary(binary) => self.analyze_binary_expression(binary),
            Expression::Range(range) => self.analyze_range_expression(range),
        }
    }

    fn analyze_identifier_expression(&self, identifier: &Identifier) -> Result<LangType, String> {
        let name = identifier.name.clone();
        let line = identifier.token.position.line;
        let column = identifier.token.position.column;

        match self.scope.get_symbol(&name) {
            None => Err("".to_string()),
            Some(symbol) => match symbol.kind {
                SymbolKind::Type | SymbolKind::Function(_) => Err("".to_string()),
                _ => Ok(symbol.symbol_type.as_ref().unwrap().clone()),
            },
        }
    }

    fn analyze_function_call_expression(&self, call: &FunctionCall) -> Result<LangType, String> {
        let identifier_name = call.identifier.name.clone();
        let line = call.identifier.token.position.line;
        let column = call.identifier.token.position.column;

        if !self.scope.get_symbol(&identifier_name).is_some() {
            Err("".to_string())
        } else {
            let symbol = self.scope.get_symbol(&identifier_name).unwrap();

            match &symbol.kind {
                SymbolKind::Function(params) => {
                    if params.len() != call.params.expressions.len() {
                        Err("".to_string())
                    } else {
                        for i in 0..params.len() {
                            let expected_type_name = params.get(i).unwrap().clone();

                            let expression = call.params.expressions.get(i).unwrap().clone();
                            let found_type_name = self.analyze_expression(&expression)?;

                            if expected_type_name != found_type_name {
                                return Err("".to_string());
                            }
                        }

                        Ok(symbol.symbol_type.as_ref().unwrap().clone())
                    }
                }
                _ => Err("".to_string()),
            }
        }
    }

    fn analyze_literal_expression(&self, literal: &Literal) -> Result<LangType, String> {
        match literal {
            Literal::String(_) => Ok("string".to_string()),
            Literal::Char(_) => Ok("char".to_string()),
            Literal::Boolean(_) => Ok("bool".to_string()),
            Literal::Number(token) => {
                if token.value.contains(".") {
                    Ok("f32".to_string())
                } else {
                    Ok("i32".to_string())
                }
            }
        }
    }

    fn analyze_unary_expression(&self, unary: &Unary) -> Result<LangType, String> {
        let expression_return_type = self.analyze_expression(&unary.expression)?;

        match unary.operator.token.kind {
            TokenKind::Tilde => {
                if !is_integer(&expression_return_type) {
                    Err("".to_string())
                } else {
                    Ok(expression_return_type)
                }
            }
            TokenKind::Plus | TokenKind::Minus => {
                if !is_number(&expression_return_type) {
                    Err("".to_string())
                } else {
                    Ok(expression_return_type)
                }
            }
            TokenKind::Exclamation => {
                if expression_return_type != "bool" {
                    Err("".to_string())
                } else {
                    Ok("bool".to_string())
                }
            }
            _ => unreachable!(),
        }
    }

    fn analyze_parenthesized_expression(
        &self,
        parenthesized: &Parenthesized,
    ) -> Result<LangType, String> {
        self.analyze_expression(&parenthesized.expression)
    }

    fn analyze_binary_expression(&self, binary: &Binary) -> Result<LangType, String> {
        let left_return_type = self.analyze_expression(&binary.left)?;
        let right_return_type = self.analyze_expression(&binary.right)?;

        match binary.operator.token.kind {
            TokenKind::EqualsEquals | TokenKind::ExclamationEquals => {
                if left_return_type != right_return_type {
                    Err("".to_string())
                } else {
                    Ok("bool".to_string())
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
                    Err("".to_string())
                } else {
                    let return_type =
                        number_type_precedence(vec![&left_return_type, &right_return_type])
                            .to_string();

                    Ok(return_type)
                }
            }
            TokenKind::GreaterThan
            | TokenKind::GreaterThanEquals
            | TokenKind::LessThan
            | TokenKind::LessThanEquals => {
                if !is_number(&left_return_type) || !is_number(&right_return_type) {
                    Err("".to_string())
                } else {
                    Ok("bool".to_string())
                }
            }
            TokenKind::AmpersandAmpersand | TokenKind::PipePipe => {
                if left_return_type != "bool" || right_return_type != "bool" {
                    Err("".to_string())
                } else {
                    Ok("bool".to_string())
                }
            }
            _ => unreachable!(),
        }
    }

    fn analyze_range_expression(&self, range: &Range) -> Result<LangType, String> {
        let left_return_type = self.analyze_expression(&range.left)?;
        let right_return_type = self.analyze_expression(&range.right)?;

        match range.operator.token.kind {
            TokenKind::DotDot | TokenKind::DotDotEquals => {
                if !is_number(&left_return_type) || !is_number(&right_return_type) {
                    Err("".to_string())
                } else {
                    Ok("range".to_string())
                }
            }
            _ => unreachable!(),
        }
    }
}
