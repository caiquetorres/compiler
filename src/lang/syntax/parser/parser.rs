use super::compilation_unit::CompilationUnit;
use super::expressions::binary::{Binary, BinaryOperator};
use super::expressions::expression::Expression;
use super::expressions::literal::Literal;
use super::expressions::parenthesized::Parenthesized;
use super::expressions::range::{Range, RangeOperator};
use super::expressions::unary::{Unary, UnaryOperator};
use super::shared::assignment_operator::AssignmentOperator;
use super::shared::block::Block;
use super::shared::function_call::{FunctionCall, Params};
use super::shared::identifier::Identifier;
use super::statements::assignment::Assignment;
use super::statements::do_while::DoWhile;
use super::statements::r#const::Const;
use super::statements::r#for::For;
use super::statements::r#if::{Else, If};
use super::statements::r#let::Let;
use super::statements::r#return::Return;
use super::statements::r#while::While;
use super::statements::statement::Statement;
use super::top_level_statements::function::{Function, ParamDeclaration, ParamsDeclaration};
use super::top_level_statements::top_level_statement::TopLevelStatement;
use crate::lang::syntax::lexer::lexer::Lexer;
use crate::lang::syntax::lexer::token::Token;
use crate::lang::syntax::lexer::token_kind::TokenKind;
use std::collections::{HashSet, VecDeque};
use std::u32::MAX;

pub struct Parser {
    tokens: VecDeque<Token>,
}

impl Parser {
    pub fn from_tokens(tokens: Vec<Token>) -> Self {
        let tokens = VecDeque::from_iter(tokens);
        Self { tokens }
    }

    pub fn from_code(code: &str) -> Self {
        let mut lexer = Lexer::new(code);
        let mut tokens: Vec<Token> = vec![];

        let mut token = lexer.next();

        while token.kind != TokenKind::EndOfFile {
            if token.kind != TokenKind::WhiteSpace {
                tokens.push(token);
            }
            token = lexer.next();
        }
        tokens.push(token);

        Self {
            tokens: VecDeque::from_iter(tokens),
        }
    }

    fn get_current_token(&self) -> &Token {
        self.tokens.get(0).unwrap()
    }

    fn use_token(&mut self, kinds: &[TokenKind]) -> Result<Token, String> {
        let token = self.next_token();

        if kinds.len() == 1 {
            if token.kind != kinds[0] {
                return Err(format!(
                    "Expected a token of type '{}' at Line {} and Column {}",
                    kinds[0], token.position.line, token.position.column
                ));
            }
        } else {
            if !kinds.iter().any(|&kind| token.kind == kind) {
                return Err(format!(
                    "Expected a token of types '{:?}' at Line {} and Column {}",
                    kinds, token.position.line, token.position.column
                ));
            }
        }

        Ok(token)
    }

    fn next_token(&mut self) -> Token {
        self.tokens.pop_front().unwrap()
    }

    pub fn parse(&mut self) -> Result<CompilationUnit, String> {
        let bad_token = self
            .tokens
            .iter()
            .find(|token| token.kind == TokenKind::Bad);

        if let Some(bad_token) = bad_token {
            return Err(format!(
                "Invalid token found at Line {} and Column {}",
                bad_token.position.line, bad_token.position.column
            ));
        }

        let mut statements: Vec<TopLevelStatement> = vec![];

        let mut current_token = self.get_current_token();

        while current_token.kind != TokenKind::EndOfFile {
            let statement = self.parse_top_level_statement()?;
            statements.push(statement);
            current_token = self.get_current_token();
        }

        return Ok(CompilationUnit::new(statements));
    }

    /// Parses a top-level statement.
    ///
    /// # Returns
    /// - `Ok(TopLevelStatement)`: Parsed top-level statement.
    /// - `Err(String)`: Error message if parsing fails.
    fn parse_top_level_statement(&mut self) -> Result<TopLevelStatement, String> {
        let current_token = self.get_current_token();

        match current_token.kind {
            TokenKind::Fun => self.parse_function_declaration(),
            _ => Err(format!("Top-level statement expected")),
        }
    }

    fn parse_type(&mut self) -> Result<Option<Identifier>, String> {
        // TODO: Apply in const and let declarations

        let current_token = self.get_current_token();

        match current_token.kind {
            TokenKind::Colon => {
                self.use_token(&[TokenKind::Colon])?;
                let type_identifier_token = self.use_token(&[TokenKind::Identifier])?;

                Ok(Some(Identifier::new(type_identifier_token)))
            }
            _ => Ok(None),
        }
    }

    /// Parses a function declaration in the format: `fun id(params) { ... }`.
    ///
    /// # Returns
    /// - `Ok(TopLevelStatement)`: Parsed function declaration as a top-level statement.
    /// - `Err(String)`: Error message if parsing fails.
    fn parse_function_declaration(&mut self) -> Result<TopLevelStatement, String> {
        self.use_token(&[TokenKind::Fun])?;

        let identifier_token = self.use_token(&[TokenKind::Identifier])?;

        self.use_token(&[TokenKind::OpenParenthesis])?;

        let params: Vec<ParamDeclaration> = self.parse_params_declaration()?;

        self.use_token(&[TokenKind::CloseParenthesis])?;

        let identifier_type = self.parse_type()?;
        let block = self.parse_block_statement()?;

        Ok(TopLevelStatement::Function(Function::new(
            Identifier::new(identifier_token),
            ParamsDeclaration(params),
            identifier_type,
            block,
        )))
    }

    /// Parses a list of parameter declarations in the format: `id : type_id, id2 : type_id2, ...`.
    ///
    /// # Returns
    /// - `Ok(Vec<ParamDeclaration>)`: Parsed parameter declarations.
    /// - `Err(String)`: Parsing error message.
    fn parse_params_declaration(&mut self) -> Result<Vec<ParamDeclaration>, String> {
        let current_token = self.get_current_token();

        match current_token.kind {
            TokenKind::Identifier => {
                let mut params: Vec<ParamDeclaration> = vec![];

                let param = self.parse_param_declaration()?;
                params.push(param);

                while self.get_current_token().kind == TokenKind::Comma {
                    self.use_token(&[TokenKind::Comma])?;

                    let param = self.parse_param_declaration()?;
                    params.push(param);
                }

                Ok(params)
            }
            _ => Ok(vec![]),
        }
    }

    /// Parses a parameter declaration in the form: `id : type_id`.
    ///
    /// # Returns
    /// - `Ok(ParamDeclaration)`: Parsed parameter declaration.
    /// - `Err(String)`: Parsing error message.
    fn parse_param_declaration(&mut self) -> Result<ParamDeclaration, String> {
        let param_name_token = self.use_token(&[TokenKind::Identifier])?;

        self.use_token(&[TokenKind::Colon])?;

        let type_id_token = self.use_token(&[TokenKind::Identifier])?;

        Ok(ParamDeclaration::new(
            Identifier::new(param_name_token),
            Identifier::new(type_id_token),
        ))
    }

    /// Parses a statement within the code.
    ///
    /// # Returns
    /// - `Ok(Statement)`: Parsed statement.
    /// - `Err(String)`: Error message if parsing fails.
    fn parse_statement(&mut self) -> Result<Statement, String> {
        let current_token = self.get_current_token();

        match current_token.kind {
            TokenKind::For => self.parse_for_statement(),
            TokenKind::Do => self.parse_do_while_statement(),
            TokenKind::While => self.parse_while_statement(),
            TokenKind::If => self.parse_if_statement(),
            TokenKind::OpenBraces => self
                .parse_block_statement()
                .map(|block| Statement::Block(block)),
            TokenKind::Identifier => {
                let identifier_token = self.next_token();
                let current_token = self.get_current_token();

                match current_token.kind {
                    TokenKind::OpenParenthesis => {
                        self.parse_function_call_statement(identifier_token)
                    }
                    TokenKind::Equals
                    | TokenKind::AmpersandEquals
                    | TokenKind::PipeEquals
                    | TokenKind::PlusEquals
                    | TokenKind::MinusEquals
                    | TokenKind::StarEquals
                    | TokenKind::SlashEquals
                    | TokenKind::ModEquals
                    | TokenKind::CircumflexEquals => {
                        self.parse_assignment_statement(identifier_token)
                    }
                    _ => Err(format!("Assignment operator or function call expected",)),
                }
            }
            TokenKind::Let => Ok(self.parse_variable_declaration_statement()?),
            TokenKind::Const => Ok(self.parse_constant_declaration_statement()?),
            TokenKind::Return => self.parse_return_statement(),
            _ => Err("Statement expected".to_string()),
        }
    }

    /// Parses a 'return' statement.
    ///
    /// # Returns
    /// - `Ok(Statement)`: Parsed 'return' statement.
    /// - `Err(String)`: Error message if parsing fails.
    fn parse_return_statement(&mut self) -> Result<Statement, String> {
        self.use_token(&[TokenKind::Return])?;

        let current_token = self.get_current_token();

        match current_token.kind {
            TokenKind::Semicolon => {
                self.use_token(&[TokenKind::Semicolon])?;

                Ok(Statement::Return(Return::new(None)))
            }
            _ => {
                let expression = self.parse_expression(0)?;

                self.use_token(&[TokenKind::Semicolon])?;

                Ok(Statement::Return(Return::new(Some(expression))))
            }
        }
    }

    /// Parses a block of statements enclosed within braces `{}`.
    ///
    /// # Returns
    /// - `Ok(Block)`: Parsed block of statements.
    /// - `Err(String)`: Error message if parsing fails.
    fn parse_block_statement(&mut self) -> Result<Block, String> {
        self.use_token(&[TokenKind::OpenBraces])?;

        let mut statements: Vec<Statement> = vec![];
        let mut current_token = self.get_current_token();

        while current_token.kind != TokenKind::CloseBraces {
            let statement = self.parse_statement()?;
            statements.push(statement);

            current_token = self.get_current_token();
        }

        self.use_token(&[TokenKind::CloseBraces])?;

        Ok(Block::new(statements))
    }

    /// Parses a 'while' loop statement in the format: `while condition { statement }`.
    ///
    /// # Returns
    /// - `Ok(Statement)`: Parsed 'while' loop statement.
    /// - `Err(String)`: Error message if parsing fails.
    fn parse_while_statement(&mut self) -> Result<Statement, String> {
        self.use_token(&[TokenKind::While])?;

        let expression = self.parse_expression(0)?;
        let statement = self.parse_statement()?;

        Ok(Statement::While(While(expression, Box::new(statement))))
    }

    /// Parses a 'while' loop statement in the format: `while condition { statement }`.
    ///
    /// # Returns
    /// - `Ok(Statement)`: Parsed 'while' loop statement.
    /// - `Err(String)`: Error message if parsing fails.
    fn parse_do_while_statement(&mut self) -> Result<Statement, String> {
        self.use_token(&[TokenKind::Do])?;

        let statement = self.parse_statement()?;

        self.use_token(&[TokenKind::While])?;

        let expression = self.parse_expression(0)?;

        self.use_token(&[TokenKind::Semicolon])?;

        Ok(Statement::DoWhile(DoWhile::new(statement, expression)))
    }

    /// Parses a 'for' loop statement in the format: `for condition in expression { statement }`.
    ///
    /// # Returns
    /// - `Ok(Statement)`: Parsed 'for' loop statement.
    /// - `Err(String)`: Error message if parsing fails.
    fn parse_for_statement(&mut self) -> Result<Statement, String> {
        self.use_token(&[TokenKind::For])?;

        let identifier_token = self.use_token(&[TokenKind::Identifier])?;

        self.use_token(&[TokenKind::In])?;

        let expression = self.parse_expression(0)?;
        let statement = self.parse_statement()?;

        Ok(Statement::For(For::new(
            Identifier::new(identifier_token),
            expression,
            Box::new(statement),
        )))
    }

    /// Parses an 'if' statement in the format: `if condition { statement } [else { else_statement }]`.
    ///
    /// Note that else blocks are optional.
    ///
    /// # Returns
    /// - `Ok(Statement)`: Parsed 'if' statement.
    /// - `Err(String)`: Error message if parsing fails.
    fn parse_if_statement(&mut self) -> Result<Statement, String> {
        self.use_token(&[TokenKind::If])?;

        let expression = self.parse_expression(0)?;
        let statement = self.parse_statement()?;

        let current_token = self.get_current_token();

        match current_token.kind {
            TokenKind::Else => {
                self.use_token(&[TokenKind::Else])?;

                let else_statement = self.parse_statement()?;

                Ok(Statement::If(If::new(
                    expression,
                    statement,
                    Some(Else(Box::new(else_statement))),
                )))
            }
            _ => Ok(Statement::If(If::new(expression, statement, None))),
        }
    }

    /// Parses a function call statement given an identifier.
    ///
    /// # Arguments
    /// - `identifier`: Token representing the function identifier.
    ///
    /// # Returns
    /// - `Ok(Statement)`: Parsed function call statement.
    /// - `Err(String)`: Error message if parsing fails.
    fn parse_function_call_statement(
        &mut self,
        identifier_token: Token,
    ) -> Result<Statement, String> {
        self.use_token(&[TokenKind::OpenParenthesis])?;

        let params = self.parse_params()?;

        self.use_token(&[TokenKind::CloseParenthesis])?;
        self.use_token(&[TokenKind::Semicolon])?;

        Ok(Statement::FunctionCall(FunctionCall::new(
            Identifier::new(identifier_token),
            params,
        )))
    }

    /// Parses a function call expression given an identifier.
    ///
    /// # Arguments
    /// - `identifier`: Token representing the function identifier.
    ///
    /// # Returns
    /// - `Ok(Statement)`: Parsed function call expression.
    /// - `Err(String)`: Error message if parsing fails.
    fn parse_function_call_expression(
        &mut self,
        identifier_token: Token,
    ) -> Result<Expression, String> {
        self.use_token(&[TokenKind::OpenParenthesis])?;

        let params = self.parse_params()?;

        self.use_token(&[TokenKind::CloseParenthesis])?;

        Ok(Expression::FunctionCall(FunctionCall::new(
            Identifier::new(identifier_token),
            params,
        )))
    }

    /// Parses a function call statement given an identifier.
    ///
    /// # Arguments
    /// - `identifier`: Token representing the function identifier.
    ///
    /// # Returns
    /// - `Ok(Statement)`: Parsed function call statement.
    /// - `Err(String)`: Error message if parsing fails.
    fn parse_assignment_statement(&mut self, identifier_token: Token) -> Result<Statement, String> {
        let operator_token = self.use_token(&[
            TokenKind::Equals,
            TokenKind::AmpersandEquals,
            TokenKind::PipeEquals,
            TokenKind::CircumflexEquals,
            TokenKind::TildeEquals,
            TokenKind::PlusEquals,
            TokenKind::MinusEquals,
            TokenKind::StarEquals,
            TokenKind::SlashEquals,
            TokenKind::ModEquals,
        ])?;

        let expression = self.parse_expression(0)?;

        self.use_token(&[TokenKind::Semicolon])?;

        Ok(Statement::Assignment(Assignment::new(
            Identifier::new(identifier_token),
            AssignmentOperator::new(operator_token),
            expression,
        )))
    }

    /// Parses a variable declaration statement in the format: "let id = expression;" or "let id: type = expression;" or "let id: type;".
    ///
    /// # Returns
    /// - `Ok(Statement)`: Parsed variable declaration statement.
    /// - `Err(String)`: Error message if parsing fails.
    fn parse_variable_declaration_statement(&mut self) -> Result<Statement, String> {
        self.use_token(&[TokenKind::Let])?;
        let identifier_token = self.use_token(&[TokenKind::Identifier])?;

        let current_token = self.get_current_token();

        match current_token.kind {
            TokenKind::Equals => {
                self.use_token(&[TokenKind::Equals])?;

                let expression = self.parse_expression(0)?;

                self.use_token(&[TokenKind::Semicolon])?;

                Ok(Statement::Let(Let::WithValue(
                    Identifier::new(identifier_token),
                    None,
                    expression,
                )))
            }
            TokenKind::Colon => {
                self.use_token(&[TokenKind::Colon])?;
                let type_identifier_token = self.use_token(&[TokenKind::Identifier])?;

                let current_token = self.get_current_token();

                match current_token.kind {
                    TokenKind::Semicolon => {
                        self.use_token(&[TokenKind::Semicolon])?;

                        Ok(Statement::Let(Let::WithoutValue(
                            Identifier::new(identifier_token),
                            Identifier::new(type_identifier_token),
                        )))
                    }
                    TokenKind::Equals => {
                        self.use_token(&[TokenKind::Equals])?;

                        let expression = self.parse_expression(0)?;

                        self.use_token(&[TokenKind::Semicolon])?;

                        Ok(Statement::Let(Let::WithValue(
                            Identifier::new(identifier_token),
                            Some(Identifier::new(type_identifier_token)),
                            expression,
                        )))
                    }
                    _ => Err("Semicolon or assignment operator expected".to_string()),
                }
            }
            _ => Err("Assignment operator or colon expected".to_string()),
        }
    }

    /// Parses a constant declaration statement.
    ///
    /// # Returns
    /// - `Ok(Statement)`: Parsed constant declaration statement.
    /// - `Err(String)`: Error message if parsing fails.
    fn parse_constant_declaration_statement(&mut self) -> Result<Statement, String> {
        self.use_token(&[TokenKind::Const])?;
        let identifier_token = self.use_token(&[TokenKind::Identifier])?;

        let current_token = self.get_current_token();

        match current_token.kind {
            TokenKind::Equals => {
                let assignment_token = self.use_token(&[TokenKind::Equals])?;
                let expression = self.parse_expression(0)?;

                self.use_token(&[TokenKind::Semicolon])?;

                Ok(Statement::Const(Const::new(
                    Identifier::new(identifier_token),
                    None,
                    AssignmentOperator::new(assignment_token),
                    expression,
                )))
            }
            TokenKind::Colon => {
                self.use_token(&[TokenKind::Colon])?;
                let type_identifier_token = self.use_token(&[TokenKind::Identifier])?;

                let assignment_token = self.use_token(&[TokenKind::Equals])?;
                let expression = self.parse_expression(0)?;

                self.use_token(&[TokenKind::Semicolon])?;

                Ok(Statement::Const(Const::new(
                    Identifier::new(identifier_token),
                    Some(Identifier::new(type_identifier_token)),
                    AssignmentOperator::new(assignment_token),
                    expression,
                )))
            }
            _ => Err("Assignment operator or colon expected".to_string()),
        }
    }

    /// Parses an expression.
    ///
    /// # Arguments
    /// - `parent_precedence`: The precedence level of the parent expression. Used to handle operator precedence and associativity.
    ///
    /// # Returns
    /// - `Ok(Expression)`: Parsed expression.
    /// - `Err(String)`: Error message if parsing fails.
    fn parse_expression(&mut self, parent_precedence: u32) -> Result<Expression, String> {
        let mut left_expression: Expression;
        let current_token = self.get_current_token();

        // Checks whether the current operator is unary or not.
        if is_unary_operator(current_token.kind) {
            // If so, the expression is going to be parsed setting its
            // precedence as max as possible, since the unary operator has
            // a precedence greater than any other possible operator.

            let operator_token = self.next_token();
            let expression = self.parse_expression(MAX)?;

            left_expression =
                Expression::Unary(Unary::new(UnaryOperator(operator_token), expression));
        } else {
            left_expression = self.parse_factor()?;
        }

        let current_token = self.get_current_token();

        // Here we break the parsing due the token is not an operator, that
        // means that the expression has over.
        if !is_binary_operator(current_token.kind) {
            return Ok(left_expression);
        }

        let mut precedence = get_binary_operator_precedence(current_token.kind);

        while precedence > parent_precedence {
            let operator_token = self.next_token();

            match operator_token.kind {
                TokenKind::DotDot | TokenKind::DotDotEquals => {
                    let range_operator = RangeOperator(operator_token);
                    let right_expression = self.parse_expression(precedence)?;

                    left_expression = Expression::Range(Range::new(
                        left_expression,
                        range_operator,
                        right_expression,
                    ));

                    let current_token = self.get_current_token();

                    if !is_binary_operator(current_token.kind) {
                        break;
                    }

                    precedence = get_binary_operator_precedence(current_token.kind);
                }
                _ => {
                    let binary_operator = BinaryOperator(operator_token);
                    let right_expression = self.parse_expression(precedence)?;

                    left_expression = Expression::Binary(Binary::new(
                        left_expression,
                        binary_operator,
                        right_expression,
                    ));

                    let current_token = self.get_current_token();

                    if !is_binary_operator(current_token.kind) {
                        break;
                    }

                    precedence = get_binary_operator_precedence(current_token.kind);
                }
            }
        }

        Ok(left_expression)
    }

    /// Parses parameters within a function or method call.
    ///
    /// # Returns
    /// - `Ok(Params)`: Parsed parameters.
    /// - `Err(String)`: Error message if parsing fails.
    fn parse_params(&mut self) -> Result<Params, String> {
        let mut expressions: Vec<Expression> = vec![];

        let expression = self.parse_expression(0)?;
        expressions.push(expression);

        let mut current_token = self.get_current_token();

        while current_token.kind != TokenKind::CloseParenthesis {
            self.use_token(&[TokenKind::Comma])?;

            let expression = self.parse_expression(0)?;
            expressions.push(expression);

            current_token = self.get_current_token();
        }

        Ok(Params::new(expressions))
    }

    fn parse_factor(&mut self) -> Result<Expression, String> {
        let token = self.next_token();

        match token.kind {
            TokenKind::Boolean => Ok(Expression::Literal(Literal::Boolean(token))),
            TokenKind::Char => Ok(Expression::Literal(Literal::Char(token))),
            TokenKind::String => Ok(Expression::Literal(Literal::String(token))),
            TokenKind::Number => Ok(Expression::Literal(Literal::Number(token))),
            TokenKind::Identifier => match self.get_current_token().kind {
                TokenKind::OpenParenthesis => self.parse_function_call_expression(token),
                _ => Ok(Expression::Identifier(Identifier::new(token))),
            },
            TokenKind::OpenParenthesis => {
                let expression = self.parse_expression(0)?;
                self.use_token(&[TokenKind::CloseParenthesis])?;

                Ok(Expression::Parenthesized(Parenthesized(Box::new(
                    expression,
                ))))
            }
            _ => Err("Expression expected".to_string()),
        }
    }
}

fn is_unary_operator(kind: TokenKind) -> bool {
    matches!(
        kind,
        TokenKind::Plus | TokenKind::Minus | TokenKind::Exclamation | TokenKind::Tilde
    )
}

fn is_binary_operator(kind: TokenKind) -> bool {
    matches!(
        kind,
        TokenKind::Slash
            | TokenKind::Star
            | TokenKind::Mod
            | TokenKind::Minus
            | TokenKind::Plus
            | TokenKind::GreaterThan
            | TokenKind::GreaterThanEquals
            | TokenKind::LessThan
            | TokenKind::LessThanEquals
            | TokenKind::Equals
            | TokenKind::EqualsEquals
            | TokenKind::Ampersand
            | TokenKind::Circumflex
            | TokenKind::Pipe
            | TokenKind::AmpersandAmpersand
            | TokenKind::PipePipe
            | TokenKind::DotDot
            | TokenKind::DotDotEquals
    )
}

macro_rules! hashset {
    { $( $x:expr ),* } => {
        HashSet::from_iter([ $( &$x ),* ].iter().cloned())
    };
}

fn get_binary_operator_precedence(kind: TokenKind) -> u32 {
    let groups: Vec<HashSet<&TokenKind>> = vec![
        hashset! {TokenKind::Slash, TokenKind::Star, TokenKind::Mod},
        hashset! {TokenKind::Plus, TokenKind::Minus},
        hashset! {TokenKind::GreaterThan, TokenKind::GreaterThanEquals, TokenKind::LessThan, TokenKind::LessThanEquals},
        hashset! {TokenKind::Equals, TokenKind::EqualsEquals},
        hashset! {TokenKind::Ampersand},
        hashset! {TokenKind::Circumflex},
        hashset! {TokenKind::Pipe},
        hashset! {TokenKind::AmpersandAmpersand},
        hashset! {TokenKind::PipePipe},
        hashset! {TokenKind::DotDot, TokenKind::DotDotEquals},
    ];

    for (pos, hash_set) in groups.iter().enumerate() {
        if hash_set.contains(&kind) {
            return (groups.len() - pos) as u32;
        }
    }

    panic!("The provided token is not a binary operator");
}

#[cfg(test)]
mod tests {
    use super::Parser;
    use crate::lang::syntax::{
        lexer::{
            token::{Position, Token},
            token_kind::TokenKind,
        },
        parser::{
            expressions::expression::Expression,
            statements::{r#let::Let, statement::Statement},
            top_level_statements::top_level_statement::TopLevelStatement,
        },
    };

    #[test]
    fn test_top_level_statement() {
        let code = " fun main() { } ";
        let mut parser = Parser::from_code(code);

        let result = parser.parse_top_level_statement();

        assert!(result.is_ok());

        match result {
            Ok(statement) => {
                assert!(matches!(statement, TopLevelStatement::Function(_)));
            }
            Err(_) => {}
        }
    }

    #[test]
    fn test_function_declaration() {
        let code = " fun main() { } ";
        let mut parser = Parser::from_code(code);

        let result = parser.parse_function_declaration();

        assert!(result.is_ok());

        match result {
            Ok(statement) => match statement {
                TopLevelStatement::Function(fun) => {
                    assert_eq!(fun.identifier.name, "main");
                    assert_eq!(fun.params_declaration.0.len(), 0);
                    assert!(fun.type_identifier.is_none());
                }
            },
            Err(_) => {}
        }

        let code = " fun main(): string { } ";
        let mut parser = Parser::from_code(code);

        let result = parser.parse_function_declaration();

        assert!(result.is_ok());

        match result {
            Ok(statement) => match statement {
                TopLevelStatement::Function(fun) => {
                    assert_eq!(fun.identifier.name, "main");
                    assert_eq!(fun.params_declaration.0.len(), 0);
                    assert!(fun.type_identifier.is_some());
                }
            },
            Err(_) => {}
        }
    }

    #[test]
    fn test_params_declaration() {
        let code = " a: i32, b: string, c: char ";
        let mut parser = Parser::from_code(code);

        let result = parser.parse_params_declaration();

        assert!(result.is_ok());

        match result {
            Ok(params) => assert_eq!(params.len(), 3),
            Err(_) => {}
        }
    }

    #[test]
    fn test_param_declaration() {
        let code = " a: i32 ";
        let mut parser = Parser::from_code(code);

        let result = parser.parse_param_declaration();

        assert!(result.is_ok());

        match result {
            Ok(param) => {
                assert_eq!(param.identifier.name, "a");
                assert_eq!(param.type_identifier.name, "i32")
            }
            Err(_) => {}
        }
    }

    #[test]
    fn test_type() {
        let code = " : i32 ";
        let mut parser = Parser::from_code(code);

        let result = parser.parse_type();

        assert!(result.is_ok());

        match result {
            Ok(param) => {
                assert!(param.is_some());
                match param {
                    Some(type_identifier) => {
                        assert_eq!(type_identifier.name, "i32")
                    }
                    None => {}
                }
            }
            Err(_) => {}
        }
    }

    #[test]
    fn test_block() {
        let code = " { { } } ";
        let mut parser = Parser::from_code(code);

        let result = parser.parse_block_statement();
        assert!(result.is_ok());

        match result {
            Ok(block) => assert_eq!(block.statements.len(), 1),
            Err(_) => {}
        }
    }

    #[test]
    fn test_statement() {
        let code = " let x = 2; ";
        let mut parser = Parser::from_code(code);

        let result = parser.parse_statement();
        assert!(result.is_ok());

        match result {
            Ok(statement) => assert!(matches!(statement, Statement::Let(_))),
            Err(_) => {}
        }

        let code = " x += 3; ";
        let mut parser = Parser::from_code(code);

        let result = parser.parse_statement();
        assert!(result.is_ok());

        match result {
            Ok(statement) => assert!(matches!(statement, Statement::Assignment(_))),
            Err(_) => {}
        }
    }

    #[test]
    fn test_for_expression() {
        let code = " for i in 2..0 {  } ";
        let mut parser = Parser::from_code(code);

        let result = parser.parse_for_statement();

        assert!(result.is_ok());

        match result {
            Ok(statement) => {
                assert!(matches!(statement, Statement::For(_)));

                match statement {
                    Statement::For(r#for) => {
                        assert_eq!(r#for.identifier.name, "i");
                        assert!(matches!(r#for.expression, Expression::Range(_)))
                    }
                    _ => {}
                };
            }
            Err(_) => {}
        }
    }

    #[test]
    fn test_do_while_statement() {
        let code = " do { x = 2; } while a; ";
        let mut parser = Parser::from_code(code);

        let result = parser.parse_do_while_statement();
        assert!(result.is_ok());

        match result {
            Ok(statement) => assert!(matches!(statement, Statement::DoWhile(_))),
            Err(_) => {}
        }
    }

    #[test]
    fn test_while_statement() {
        let code = " while a { x = 2; } ";
        let mut parser = Parser::from_code(code);

        let result = parser.parse_while_statement();
        assert!(result.is_ok());

        match result {
            Ok(statement) => assert!(matches!(statement, Statement::While(_))),
            Err(_) => {}
        }
    }

    #[test]
    fn test_if_statement() {
        let code = " if a == 2 { a += 2; } ";
        let mut parser = Parser::from_code(code);

        let result = parser.parse_if_statement();
        assert!(result.is_ok());

        match result {
            Ok(statement) => {
                assert!(matches!(statement, Statement::If(_)));
                match statement {
                    Statement::If(r#if) => assert!(r#if.r#else.is_none()),
                    _ => {}
                }
            }
            Err(_) => {}
        }
    }

    #[test]
    fn test_function_call_statement() {
        let code = " (a, b); ";
        let mut parser = Parser::from_code(code);

        let identifier = Token::new(TokenKind::Identifier, Position::new(0, 0, 0), "fun");

        let result = parser.parse_function_call_statement(identifier);
        assert!(result.is_ok());

        match result {
            Ok(statement) => {
                assert!(matches!(statement, Statement::FunctionCall(_)));
                match statement {
                    Statement::FunctionCall(fun) => {
                        assert_eq!(fun.identifier.name, "fun");
                        assert_eq!(fun.params.expressions.len(), 2);
                    }
                    _ => {}
                }
            }
            Err(_) => {}
        }
    }

    #[test]
    fn test_assignment_statement() {
        let code = " += 2; ";
        let mut parser = Parser::from_code(code);

        let identifier = Token::new(TokenKind::Identifier, Position::new(0, 0, 0), "a");

        let result = parser.parse_assignment_statement(identifier);
        assert!(result.is_ok());

        match result {
            Ok(statement) => {
                assert!(matches!(statement, Statement::Assignment(_)));
                match statement {
                    Statement::Assignment(assignment) => {
                        assert_eq!(assignment.identifier.name, "a");
                        assert_eq!(assignment.operator.name, "+=");
                    }
                    _ => {}
                }
            }
            Err(_) => {}
        }
    }

    #[test]
    fn test_variable_declaration_statement() {
        let code = " let x = 2; ";
        let mut parser = Parser::from_code(code);

        let result = parser.parse_variable_declaration_statement();
        assert!(result.is_ok());

        match result {
            Ok(statement) => {
                assert!(matches!(statement, Statement::Let(_)));
                match statement {
                    Statement::Let(r#let) => {
                        assert!(matches!(r#let, Let::WithValue(_, _, _)));
                        match r#let {
                            Let::WithValue(identifier, type_identifier, _) => {
                                assert_eq!(identifier.name, "x");
                                assert!(type_identifier.is_none());
                            }
                            _ => {}
                        }
                    }
                    _ => {}
                }
            }
            Err(_) => {}
        }

        let code = " let x:i32 = 2; ";
        let mut parser = Parser::from_code(code);

        let result = parser.parse_variable_declaration_statement();
        assert!(result.is_ok());

        match result {
            Ok(statement) => {
                assert!(matches!(statement, Statement::Let(_)));
                match statement {
                    Statement::Let(r#let) => {
                        assert!(matches!(r#let, Let::WithValue(_, _, _)));
                        match r#let {
                            Let::WithValue(identifier, type_identifier, _) => {
                                assert_eq!(identifier.name, "x");
                                assert!(type_identifier.is_some());
                            }
                            _ => {}
                        }
                    }
                    _ => {}
                }
            }
            Err(_) => {}
        }
    }

    #[test]
    fn test_const_declaration_statement() {
        let code = " const x = 2; ";
        let mut parser = Parser::from_code(code);

        let result = parser.parse_constant_declaration_statement();
        assert!(result.is_ok());

        match result {
            Ok(statement) => {
                assert!(matches!(statement, Statement::Const(_)));
                match statement {
                    Statement::Const(r#const) => {
                        assert_eq!(r#const.identifier.name, "x");
                        assert!(r#const.type_identifier.is_none());
                    }
                    _ => {}
                }
            }
            Err(_) => {}
        }

        let code = " const x: i32 = 2; ";
        let mut parser = Parser::from_code(code);

        let result = parser.parse_constant_declaration_statement();
        assert!(result.is_ok());

        match result {
            Ok(statement) => {
                assert!(matches!(statement, Statement::Const(_)));
                match statement {
                    Statement::Const(r#const) => {
                        assert_eq!(r#const.identifier.name, "x");
                        assert!(r#const.type_identifier.is_some());
                    }
                    _ => {}
                }
            }
            Err(_) => {}
        }
    }

    #[test]
    fn test_return_statement() {
        let code = " return; ";
        let mut parser = Parser::from_code(code);

        let result = parser.parse_return_statement();
        assert!(result.is_ok());

        match result {
            Ok(statement) => {
                assert!(matches!(statement, Statement::Return(_)));
                match statement {
                    Statement::Return(r#return) => {
                        assert!(r#return.expression.is_none());
                    }
                    _ => {}
                }
            }
            Err(_) => {}
        }

        let code = " return 2; ";
        let mut parser = Parser::from_code(code);

        let result = parser.parse_return_statement();
        assert!(result.is_ok());

        match result {
            Ok(statement) => {
                assert!(matches!(statement, Statement::Return(_)));
                match statement {
                    Statement::Return(r#return) => {
                        assert!(r#return.expression.is_some());
                    }
                    _ => {}
                }
            }
            Err(_) => {}
        }
    }

    #[test]
    fn test_expression() {
        let code = " (1 + 2) * 3 ";
        let mut parser = Parser::from_code(code);

        let result = parser.parse_expression(0);
        assert!(result.is_ok());

        match result {
            Ok(expression) => assert!(matches!(expression, Expression::Binary(_))),
            _ => {}
        }

        let code = " a ";
        let mut parser = Parser::from_code(code);

        let result = parser.parse_expression(0);
        assert!(result.is_ok());

        match result {
            Ok(expression) => assert!(matches!(expression, Expression::Identifier(_))),
            _ => {}
        }

        let code = " 0..=3 ";
        let mut parser = Parser::from_code(code);

        let result = parser.parse_expression(0);
        assert!(result.is_ok());

        match result {
            Ok(expression) => assert!(matches!(expression, Expression::Range(_))),
            _ => {}
        }
    }
}