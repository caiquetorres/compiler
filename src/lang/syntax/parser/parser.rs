use super::{
    compilation_unit::CompilationUnit, expressions::expression::Expression,
    statements::statement::Statement, top_level_statements::function::Function,
    top_level_statements::top_level_statement::TopLevelStatement,
};
use crate::lang::syntax::lexer::{lexer::Lexer, token::Token, token_kind::TokenKind};
use crate::lang::syntax::parser::{
    expressions::{
        binary::{Binary, BinaryOperator},
        literal::Literal,
        parenthesized::Parenthesized,
        range::{Range, RangeOperator},
        unary::{Unary, UnaryOperator},
    },
    shared::{
        assignment_operator::AssignmentOperator,
        block::Block,
        function_call::{FunctionCall, Params},
        identifier::Identifier,
    },
    statements::{
        assignment::Assignment,
        do_while::DoWhile,
        r#const::Const,
        r#for::For,
        r#if::{Else, If},
        r#let::Let,
        r#return::Return,
        r#while::While,
    },
    top_level_statements::function::{ParamDeclaration, ParamsDeclaration},
};
use std::collections::VecDeque;

pub struct Parser {
    tokens: VecDeque<Token>,
}

impl Parser {
    pub fn new(text: &str) -> Self {
        let mut tokens = VecDeque::new();
        let mut lexer = Lexer::new(text);

        let mut token = lexer.next();

        while token.kind != TokenKind::EndOfFile {
            if token.kind != TokenKind::WhiteSpace {
                tokens.push_back(token);
            }
            token = lexer.next();
        }
        tokens.push_back(token);

        Self { tokens }
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

        let current_token = self.get_current_token();

        match current_token.kind {
            TokenKind::Colon => {
                self.use_token(&[TokenKind::Colon])?;

                let type_identifier_token = self.use_token(&[TokenKind::Identifier])?;
                let block = self.parse_block()?;

                Ok(TopLevelStatement::Function(Function::new(
                    Identifier::new(identifier_token),
                    ParamsDeclaration(params),
                    Some(Identifier::new(type_identifier_token)),
                    block,
                )))
            }
            TokenKind::OpenBraces => {
                let block = self.parse_block()?;

                Ok(TopLevelStatement::Function(Function::new(
                    Identifier::new(identifier_token),
                    ParamsDeclaration(params),
                    None,
                    block,
                )))
            }
            _ => Err("Type or block expected".to_string()),
        }
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

        Ok(ParamDeclaration(
            Identifier::new(param_name_token),
            Identifier::new(type_id_token),
        ))
    }

    fn parse_statement(&mut self) -> Result<Statement, String> {
        let current_token = self.get_current_token();

        match current_token.kind {
            TokenKind::For => self.parse_for_statement(),
            TokenKind::Do => self.parse_do_while_statement(),
            TokenKind::While => self.parse_while_statement(),
            TokenKind::If => self.parse_if_statement(),
            TokenKind::OpenBraces => self.parse_block().map(|block| Statement::Block(block)),
            TokenKind::Identifier => {
                let identifier_token = self.next_token();

                match self.get_current_token().kind {
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
                    | TokenKind::CircumflexEquals => self.parse_assignment(identifier_token),
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

        let token = self.get_current_token();

        match token.kind {
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
    fn parse_block(&mut self) -> Result<Block, String> {
        self.use_token(&[TokenKind::OpenBraces])?;

        let mut statements: Vec<Statement> = vec![];

        while self.get_current_token().kind != TokenKind::CloseBraces {
            let statement = self.parse_statement()?;
            statements.push(statement);
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

        Ok(Statement::DoWhile(DoWhile(Box::new(statement), expression)))
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

        Ok(Statement::For(For(
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

                Ok(Statement::If(If(
                    expression,
                    Box::new(statement),
                    Some(Else(Box::new(else_statement))),
                )))
            }
            _ => Ok(Statement::If(If(expression, Box::new(statement), None))),
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

        Ok(Statement::FunctionCall(FunctionCall(
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

        Ok(Expression::FunctionCall(FunctionCall(
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
    fn parse_assignment(&mut self, identifier_token: Token) -> Result<Statement, String> {
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

    fn parse_variable_declaration_statement(&mut self) -> Result<Statement, String> {
        self.use_token(&[TokenKind::Let])?;
        let identifier_token = self.use_token(&[TokenKind::Identifier])?;

        let current_token = self.get_current_token();

        match current_token.kind {
            TokenKind::Equals => {
                let assignment_token = self.use_token(&[TokenKind::Equals])?;
                let expression = self.parse_expression(0)?;

                self.use_token(&[TokenKind::Semicolon])?;

                Ok(Statement::Let(Let::WithValue(
                    Identifier::new(identifier_token),
                    None,
                    AssignmentOperator::new(assignment_token),
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
                        let assignment_token = self.use_token(&[TokenKind::Equals])?;
                        let expression = self.parse_expression(0)?;

                        self.use_token(&[TokenKind::Semicolon])?;

                        Ok(Statement::Let(Let::WithValue(
                            Identifier::new(identifier_token),
                            Some(Identifier::new(type_identifier_token)),
                            AssignmentOperator::new(assignment_token),
                            expression,
                        )))
                    }
                    _ => Err("Semicolon or assignment operator expected".to_string()),
                }
            }
            _ => Err("Assignment operator or colon expected".to_string()),
        }
    }

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
                self.use_token(&[TokenKind::Equals])?;

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

    fn parse_expression(&mut self, parent_precedence: u32) -> Result<Expression, String> {
        let mut left: Expression;
        let token = self.get_current_token();

        let unary_precedence = get_unary_operator_precedence(token.kind);

        if unary_precedence != 0 && unary_precedence >= parent_precedence {
            let operator_token = self.next_token();

            left = Expression::Unary(Unary::new(
                UnaryOperator(operator_token),
                self.parse_expression(unary_precedence)?,
            ));
        } else {
            left = self.parse_factor()?;
        }

        let token = self.get_current_token();
        let mut precedence = get_binary_operator_precedence(token.kind);

        while precedence != 0 && precedence > parent_precedence {
            let operator_token = self.next_token();

            if operator_token.kind == TokenKind::DotDot
                || operator_token.kind == TokenKind::DotDotEquals
            {
                let operator = RangeOperator(operator_token);
                let right = self.parse_expression(precedence)?;

                left = Expression::Range(Range::new(left, operator, right));

                precedence = get_binary_operator_precedence(self.get_current_token().kind);
            } else {
                let operator = BinaryOperator(operator_token);
                let right = self.parse_expression(precedence)?;

                left = Expression::Binary(Binary::new(left, operator, right));

                precedence = get_binary_operator_precedence(self.get_current_token().kind);
            }
        }

        Ok(left)
    }

    fn parse_params(&mut self) -> Result<Params, String> {
        match self.get_current_token().kind {
            TokenKind::CloseParenthesis => Ok(Params(vec![])),
            _ => {
                let mut expressions: Vec<Expression> = vec![];

                while self.get_current_token().kind != TokenKind::CloseParenthesis {
                    let expression = self.parse_expression(0)?;
                    expressions.push(expression);
                    self.next_token();
                }

                Ok(Params(expressions))
            }
        }
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

fn get_unary_operator_precedence(kind: TokenKind) -> u32 {
    match kind {
        TokenKind::Plus | TokenKind::Minus | TokenKind::Exclamation | TokenKind::Tilde => 11,
        _ => 0,
    }
}

fn get_binary_operator_precedence(kind: TokenKind) -> u32 {
    match kind {
        TokenKind::Slash | TokenKind::Star | TokenKind::Mod => 10,
        TokenKind::Minus | TokenKind::Plus => 9,
        TokenKind::GreaterThan
        | TokenKind::GreaterThanEquals
        | TokenKind::LessThan
        | TokenKind::LessThanEquals => 8,
        TokenKind::Equals | TokenKind::EqualsEquals => 7,
        TokenKind::Ampersand => 6,
        TokenKind::Circumflex => 5,
        TokenKind::Pipe => 4,
        TokenKind::AmpersandAmpersand => 3,
        TokenKind::PipePipe => 2,
        TokenKind::DotDot | TokenKind::DotDotEquals => 1,
        _ => 0,
    }
}

#[cfg(test)]
mod tests {
    use crate::lang::syntax::parser::{
        expressions::expression::Expression,
        statements::{r#for::For, statement::Statement},
    };

    use super::Parser;

    #[test]
    fn test_for_expression() {
        let code = "for i in 2..0 {  }";
        let mut parser = Parser::new(code);

        let statement = parser.parse_for_statement();
        assert!(statement.is_ok());

        if let Ok(for_statement) = statement {
            assert!(matches!(for_statement, Statement::For(For(_, _, _))));
            if let Statement::For(r#for) = for_statement {
                assert_eq!(r#for.0.name, "i");
            }
        }
    }

    #[test]
    fn test_parse_expression() {
        let code = "2 + 2 * 2";
        let mut parser = Parser::new(code);

        let result = parser.parse_expression(0);

        assert!(result.is_ok());

        if let Ok(binary_expression) = result {
            assert!(matches!(binary_expression, Expression::Binary(_)));
        }
    }
}
