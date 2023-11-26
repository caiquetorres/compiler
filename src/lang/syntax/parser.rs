use crate::lang::lexer::token::Token;
use crate::lang::lexer::token_kind::TokenKind;

use super::compilation_unit::CompilationUnit;
use super::expressions::array::Array;
use super::expressions::expression::ExpressionMeta;
use super::expressions::{
    binary::{Binary, BinaryOperator},
    expression::Expression,
    literal::Literal,
    parenthesized::Parenthesized,
    range::{Range, RangeOperator},
    unary::{Unary, UnaryOperator},
};

use super::shared::syntax_type::SyntaxType;
use super::shared::{
    assignment_operator::AssignmentOperator, block::Block, identifier::Identifier,
};
use super::statements::print::Print;
use super::statements::r#break::Break;
use super::statements::r#continue::Continue;
use super::statements::{
    assignment::Assignment,
    do_while::DoWhile,
    r#for::For,
    r#if::{Else, If},
    r#let::Let,
    r#return::Return,
    r#while::While,
    statement::Statement,
};
use super::syntax_error::SyntaxError;
use super::top_level_statements::{
    function::{Function, ParamDeclaration, ParamsDeclaration},
    top_level_statement::TopLevelStatement,
};

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

    #[cfg(test)]
    pub fn from_code(code: &str) -> Self {
        use crate::lang::lexer::lexer::Lexer;

        let mut lexer = Lexer::new(code);
        let tokens = lexer.lex().unwrap();

        Self {
            tokens: VecDeque::from_iter(tokens),
        }
    }

    fn get_current_token(&self) -> &Token {
        self.tokens.get(0).unwrap()
    }

    fn use_token(&mut self, kinds: &[TokenKind]) -> Result<Token, SyntaxError> {
        let current_token = self.get_current_token();
        let position = current_token.position;
        let token = self.next_token();

        if kinds.contains(&token.kind) {
            Ok(token)
        } else {
            Err(SyntaxError::UnexpectedToken {
                found: token.kind,
                position,
            })
        }
    }

    fn next_token(&mut self) -> Token {
        self.tokens.pop_front().unwrap()
    }

    pub fn parse(&mut self) -> Result<CompilationUnit, SyntaxError> {
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
    fn parse_top_level_statement(&mut self) -> Result<TopLevelStatement, SyntaxError> {
        let current_token = self.get_current_token();

        match current_token.kind {
            TokenKind::FunKeyword => self.parse_function_declaration(),
            _ => Err(SyntaxError::TopLevelStatementExpected {
                position: current_token.position,
            }),
        }
    }

    fn parse_return_type_optional(&mut self) -> Result<Option<SyntaxType>, SyntaxError> {
        let current_token = self.get_current_token();

        if let TokenKind::ArrowRight = current_token.kind {
            self.next_token();
            Ok(Some(self.parse_type()?))
        } else {
            Ok(None)
        }
    }

    fn parse_type_optional(&mut self) -> Result<Option<SyntaxType>, SyntaxError> {
        let current_token = self.get_current_token();

        if let TokenKind::Colon = current_token.kind {
            self.next_token();
            Ok(Some(self.parse_type()?))
        } else {
            Ok(None)
        }
    }

    fn parse_type(&mut self) -> Result<SyntaxType, SyntaxError> {
        let token = self.use_token(&[
            TokenKind::Identifier,
            TokenKind::LeftBracket,
            TokenKind::Ref,
            TokenKind::LeftParenthesis,
        ])?;

        match &token.kind {
            TokenKind::Identifier => {
                let type_identifier_token = token;
                Ok(SyntaxType::new_simple(type_identifier_token))
            }
            TokenKind::LeftParenthesis => {
                // (i32, i32) -> i32

                let mut params: Vec<SyntaxType> = vec![];

                if self.get_current_token().kind != TokenKind::RightParenthesis {
                    loop {
                        let r#type = self.parse_type()?;
                        params.push(r#type);

                        let current_token = self.get_current_token();
                        if current_token.kind == TokenKind::RightParenthesis {
                            break;
                        }

                        self.use_token(&[TokenKind::Comma])?;
                    }
                }

                self.use_token(&[TokenKind::RightParenthesis])?;
                self.use_token(&[TokenKind::ArrowRight])?;

                let return_type = self.parse_type()?;

                Ok(SyntaxType::new_function(
                    params,
                    return_type,
                    token.position,
                ))
            }
            TokenKind::LeftBracket => {
                let r#type = self.parse_type()?;

                self.use_token(&[TokenKind::Semicolon])?;
                let array_size_token = self.use_token(&[TokenKind::NumberLiteral])?;

                self.use_token(&[TokenKind::RightBracket])?;

                Ok(SyntaxType::new_array(
                    r#type,
                    array_size_token,
                    token.position,
                ))
            }
            TokenKind::Ref => {
                let r#type = self.parse_type()?;
                Ok(SyntaxType::new_reference(r#type, token.position))
            }
            _ => unreachable!(),
        }
    }

    /// Parses a function declaration in the format: `fun id(params) { ... }`.
    ///
    /// # Returns
    /// - `Ok(TopLevelStatement)`: Parsed function declaration as a top-level statement.
    /// - `Err(String)`: Error message if parsing fails.
    fn parse_function_declaration(&mut self) -> Result<TopLevelStatement, SyntaxError> {
        self.use_token(&[TokenKind::FunKeyword])?;

        let identifier_token = self.use_token(&[TokenKind::Identifier])?;

        self.use_token(&[TokenKind::LeftParenthesis])?;

        let params: Vec<ParamDeclaration> = self.parse_params_declaration()?;

        self.use_token(&[TokenKind::RightParenthesis])?;

        let identifier_type = self.parse_return_type_optional()?;
        let block = self.parse_block()?;

        Ok(TopLevelStatement::Function(Function::new(
            Identifier::new(identifier_token),
            ParamsDeclaration::new(params),
            identifier_type,
            block,
        )))
    }

    /// Parses a list of parameter declarations in the format: `id : type_id, id2 : type_id2, ...`.
    ///
    /// # Returns
    /// - `Ok(Vec<ParamDeclaration>)`: Parsed parameter declarations.
    /// - `Err(String)`: Parsing error message.
    fn parse_params_declaration(&mut self) -> Result<Vec<ParamDeclaration>, SyntaxError> {
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
    fn parse_param_declaration(&mut self) -> Result<ParamDeclaration, SyntaxError> {
        let param_name_token = self.use_token(&[TokenKind::Identifier])?;

        self.use_token(&[TokenKind::Colon])?;

        let r#type = self.parse_type()?;

        Ok(ParamDeclaration::new(
            Identifier::new(param_name_token),
            r#type,
        ))
    }

    /// Parses a statement within the code.
    ///
    /// # Returns
    /// - `Ok(Statement)`: Parsed statement.
    /// - `Err(SyntaxError)`: Syntax error if parsing fails.
    fn parse_statement(&mut self) -> Result<Statement, SyntaxError> {
        let current_token = self.get_current_token();

        match current_token.kind {
            TokenKind::ForKeyword => self.parse_for_statement(),
            TokenKind::DoKeyword => self.parse_do_while_statement(),
            TokenKind::WhileKeyword => self.parse_while_statement(),
            TokenKind::IfKeyword => self.parse_if_statement(),
            TokenKind::LeftBrace => self.parse_block().map(|block| Statement::Block(block)),
            TokenKind::LetKeyword => Ok(self.parse_variable_declaration_statement()?),
            TokenKind::ReturnKeyword => self.parse_return_statement(),
            TokenKind::BreakKeyword => self.parse_break_statement(),
            TokenKind::ContinueKeyword => self.parse_continue_statement(),
            TokenKind::PrintKeyword | TokenKind::PrintlnKeyword => self.parse_print_statement(),
            _ => {
                let left_expression = self.parse_expression(0)?;

                let current_token = self.get_current_token();

                match current_token.kind {
                    TokenKind::Equals
                    | TokenKind::AmpersandEquals
                    | TokenKind::PipeEquals
                    | TokenKind::CircumflexEquals
                    | TokenKind::TildeEquals
                    | TokenKind::PlusEquals
                    | TokenKind::MinusEquals
                    | TokenKind::StarEquals
                    | TokenKind::SlashEquals
                    | TokenKind::ModEquals => {
                        // Assignment

                        let current_token = self.next_token();

                        let right_expression = self.parse_expression(0)?;

                        self.use_token(&[TokenKind::Semicolon])?;

                        Ok(Statement::Assignment(Assignment::new(
                            left_expression,
                            AssignmentOperator::new(current_token),
                            right_expression,
                        )))
                    }
                    _ => {
                        // Semicolon

                        self.use_token(&[TokenKind::Semicolon])?;

                        Ok(Statement::Expression(left_expression))
                    }
                }
            }
        }
    }

    fn parse_continue_statement(&mut self) -> Result<Statement, SyntaxError> {
        let continue_token = self.use_token(&[TokenKind::ContinueKeyword])?;
        self.use_token(&[TokenKind::Semicolon])?;
        Ok(Statement::Continue(Continue::new(continue_token.position)))
    }

    fn parse_print_statement(&mut self) -> Result<Statement, SyntaxError> {
        let token = self.use_token(&[TokenKind::PrintKeyword, TokenKind::PrintlnKeyword])?;

        let new_line = token.kind == TokenKind::PrintlnKeyword;
        let mut expressions: Vec<Expression> = vec![];

        while self.get_current_token().kind != TokenKind::Semicolon {
            let expression = self.parse_expression(0)?;

            expressions.push(expression);

            let current_token = self.get_current_token();
            if current_token.kind != TokenKind::Comma {
                break;
            }
            self.next_token();
        }

        self.use_token(&[TokenKind::Semicolon])?;

        Ok(Statement::Print(Print::new(new_line, expressions)))
    }

    fn parse_break_statement(&mut self) -> Result<Statement, SyntaxError> {
        let break_token = self.use_token(&[TokenKind::BreakKeyword])?;
        self.use_token(&[TokenKind::Semicolon])?;
        Ok(Statement::Break(Break::new(break_token.position)))
    }

    /// Parses a 'return' statement.
    ///
    /// # Returns
    /// - `Ok(Statement)`: Parsed 'return' statement.
    /// - `Err(SyntaxError)`: Syntax error if parsing fails.
    fn parse_return_statement(&mut self) -> Result<Statement, SyntaxError> {
        let return_token = self.use_token(&[TokenKind::ReturnKeyword])?;

        let current_token = self.get_current_token();

        match current_token.kind {
            TokenKind::Semicolon => {
                self.use_token(&[TokenKind::Semicolon])?;

                Ok(Statement::Return(Return::new(None, return_token.position)))
            }
            _ => {
                let expression = self.parse_expression(0)?;

                self.use_token(&[TokenKind::Semicolon])?;

                Ok(Statement::Return(Return::new(
                    Some(expression),
                    return_token.position,
                )))
            }
        }
    }

    /// Parses a block of statements enclosed within braces `{}`.
    ///
    /// # Returns
    /// - `Ok(Block)`: Parsed block of statements.
    /// - `Err(SyntaxError)`: Syntax error if parsing fails.
    fn parse_block(&mut self) -> Result<Block, SyntaxError> {
        self.use_token(&[TokenKind::LeftBrace])?;

        let mut statements: Vec<Statement> = vec![];
        let mut current_token = self.get_current_token();

        while current_token.kind != TokenKind::RightBrace {
            let statement = self.parse_statement()?;
            statements.push(statement);

            current_token = self.get_current_token();
        }

        self.use_token(&[TokenKind::RightBrace])?;

        Ok(Block::new(statements))
    }

    /// Parses a 'while' loop statement in the format: `while condition { statement }`.
    ///
    /// # Returns
    /// - `Ok(Statement)`: Parsed 'while' loop statement.
    /// - `Err(SyntaxError)`: Syntax error if parsing fails.
    fn parse_while_statement(&mut self) -> Result<Statement, SyntaxError> {
        self.use_token(&[TokenKind::WhileKeyword])?;

        let expression = self.parse_expression(0)?;
        let block = self.parse_block()?;

        Ok(Statement::While(While::new(expression, block)))
    }

    /// Parses a 'while' loop statement in the format: `while condition { statement }`.
    ///
    /// # Returns
    /// - `Ok(Statement)`: Parsed 'while' loop statement.
    /// - `Err(SyntaxError)`: Syntax error if parsing fails.
    fn parse_do_while_statement(&mut self) -> Result<Statement, SyntaxError> {
        self.use_token(&[TokenKind::DoKeyword])?;

        let block = self.parse_block()?;

        self.use_token(&[TokenKind::WhileKeyword])?;

        let expression = self.parse_expression(0)?;

        self.use_token(&[TokenKind::Semicolon])?;

        Ok(Statement::DoWhile(DoWhile::new(block, expression)))
    }

    /// Parses a 'for' loop statement in the format: `for condition in expression { statement }`.
    ///
    /// # Returns
    /// - `Ok(Statement)`: Parsed 'for' loop statement.
    /// - `Err(SyntaxError)`: Syntax error if parsing fails.
    fn parse_for_statement(&mut self) -> Result<Statement, SyntaxError> {
        self.use_token(&[TokenKind::ForKeyword])?;

        let identifier_token = self.use_token(&[TokenKind::Identifier])?;

        self.use_token(&[TokenKind::InKeyword])?;

        let expression = self.parse_expression(0)?;
        let statement = self.parse_block()?;

        Ok(Statement::For(For::new(
            Identifier::new(identifier_token),
            expression,
            statement,
        )))
    }

    /// Parses an 'if' statement in the format: `if condition { statement } [else { else_statement }]`.
    ///
    /// Note that else blocks are optional.
    ///
    /// # Returns
    /// - `Ok(Statement)`: Parsed 'if' statement.
    /// - `Err(SyntaxError)`: Syntax error if parsing fails.
    fn parse_if_statement(&mut self) -> Result<Statement, SyntaxError> {
        self.use_token(&[TokenKind::IfKeyword])?;

        let expression = self.parse_expression(0)?;
        let block = self.parse_block()?;

        let current_token = self.get_current_token();

        match current_token.kind {
            TokenKind::ElseKeyword => {
                self.use_token(&[TokenKind::ElseKeyword])?;

                let else_statement = self.parse_block()?;

                Ok(Statement::If(If::new(
                    expression,
                    block,
                    Some(Else::new(else_statement)),
                )))
            }
            _ => Ok(Statement::If(If::new(expression, block, None))),
        }
    }

    /// Parses a variable declaration statement in the format: "let id = expression;" or "let id: type = expression;" or "let id: type;".
    ///
    /// # Returns
    /// - `Ok(Statement)`: Parsed variable declaration statement.
    /// - `Err(SyntaxError)`: Syntax error if parsing fails.
    fn parse_variable_declaration_statement(&mut self) -> Result<Statement, SyntaxError> {
        self.use_token(&[TokenKind::LetKeyword])?;

        let identifier_token = self.use_token(&[TokenKind::Identifier])?;

        let type_identifier = self.parse_type_optional()?;

        let current_token = self.get_current_token();

        match current_token.kind {
            TokenKind::Semicolon => {
                self.use_token(&[TokenKind::Semicolon])?;
                Ok(Statement::Let(Let::new(
                    Identifier::new(identifier_token),
                    type_identifier,
                    None,
                )))
            }
            _ => {
                self.use_token(&[TokenKind::Equals])?;

                let expression = self.parse_expression(0)?;

                self.use_token(&[TokenKind::Semicolon])?;

                Ok(Statement::Let(Let::new(
                    Identifier::new(identifier_token),
                    type_identifier,
                    Some(expression),
                )))
            }
        }
    }

    /// Parses an expression.
    ///
    /// # Arguments
    /// - `parent_precedence`: The precedence level of the parent expression. Used to handle operator precedence and associativity.
    ///
    /// # Returns
    /// - `Ok(Expression)`: Parsed expression.
    /// - `Err(SyntaxError)`: Syntax error if parsing fails.
    fn parse_expression(&mut self, parent_precedence: u32) -> Result<Expression, SyntaxError> {
        let current_token = self.get_current_token();

        let mut left_expression: Expression;

        // Checks whether the current operator is unary or not.
        if is_unary_operator(current_token.kind) {
            // If so, the expression is going to be parsed setting its
            // precedence as max as possible, since the unary operator has
            // a precedence greater than any other possible operator.

            let operator_token = self.next_token();
            let expression = self.parse_expression(MAX)?;

            left_expression =
                Expression::Unary(Unary::new(UnaryOperator::new(operator_token), expression));
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
                    let range_operator = RangeOperator::new(operator_token);
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
                    let binary_operator = BinaryOperator::new(operator_token);
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

    fn parse_expression_meta(&mut self) -> Result<Option<ExpressionMeta>, SyntaxError> {
        let mut meta: Option<ExpressionMeta> = None;

        if matches!(
            self.get_current_token().kind,
            TokenKind::LeftBracket | TokenKind::LeftParenthesis | TokenKind::Dot
        ) {
            let token = self.use_token(&[
                TokenKind::LeftBracket,
                TokenKind::LeftParenthesis,
                TokenKind::Dot,
            ])?;

            match &token.kind {
                TokenKind::LeftBracket => {
                    // a[b]

                    let expression = self.parse_expression(0)?;
                    self.use_token(&[TokenKind::RightBracket])?;

                    meta = Some(ExpressionMeta::Index(
                        Box::new(expression),
                        Box::new(self.parse_expression_meta()?),
                        token.position,
                    ));
                }
                TokenKind::LeftParenthesis => {
                    // a()

                    let mut expressions: Vec<Expression> = vec![];

                    while self.get_current_token().kind != TokenKind::RightParenthesis {
                        let expression = self.parse_expression(0)?;
                        expressions.push(expression);

                        if self.get_current_token().kind != TokenKind::RightParenthesis {
                            self.use_token(&[TokenKind::Comma])?;
                        }
                    }

                    self.use_token(&[TokenKind::RightParenthesis])?;

                    meta = Some(ExpressionMeta::Call(
                        expressions,
                        Box::new(self.parse_expression_meta()?),
                        token.position,
                    ));
                }
                _ => {}
            }
        }

        Ok(meta)
    }

    fn parse_factor(&mut self) -> Result<Expression, SyntaxError> {
        let token = self.next_token();

        match token.kind {
            TokenKind::BooleanLiteral => Ok(Expression::Literal(Literal::Boolean(token))),
            TokenKind::CharLiteral => Ok(Expression::Literal(Literal::Char(token))),
            TokenKind::StringLiteral => Ok(Expression::Literal(Literal::String(token))),
            TokenKind::NumberLiteral => Ok(Expression::Literal(Literal::Number(token))),
            TokenKind::Identifier => {
                let meta = self.parse_expression_meta()?;
                Ok(Expression::Identifier(Identifier::new(token), meta))
            }
            TokenKind::LeftBracket => {
                let mut expressions: Vec<Expression> = vec![];

                loop {
                    let expression = self.parse_expression(0)?;
                    expressions.push(expression);

                    let current_token = self.get_current_token();

                    if current_token.kind == TokenKind::RightBracket {
                        self.next_token();
                        break;
                    }

                    self.use_token(&[TokenKind::Comma])?;
                }

                Ok(Expression::Array(Array::new(expressions, token.position)))
            }
            TokenKind::LeftParenthesis => {
                let expression = self.parse_expression(0)?;
                self.use_token(&[TokenKind::RightParenthesis])?;

                let meta = self.parse_expression_meta()?;

                Ok(Expression::Parenthesized(
                    Parenthesized::new(expression, token.position),
                    meta,
                ))
            }
            _ => Err(SyntaxError::ExpressionExpected {
                position: token.position,
            }),
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
            | TokenKind::EqualsEquals
            | TokenKind::ExclamationEquals
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
        hashset! {TokenKind::EqualsEquals, TokenKind::ExclamationEquals},
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
        expressions::expression::Expression, shared::syntax_type::SyntaxType,
        statements::statement::Statement,
        top_level_statements::top_level_statement::TopLevelStatement,
    };

    #[test]
    fn test_top_level_statement() {
        let code = " fun main() { } ";
        let mut parser = Parser::from_code(code);

        let result = parser.parse_top_level_statement();

        assert!(result.is_ok());

        if let Ok(statement) = result {
            assert!(matches!(statement, TopLevelStatement::Function(_)));
        }
    }

    #[test]
    fn test_function_declaration() {
        let code = " fun main() { } ";
        let mut parser = Parser::from_code(code);

        let result = parser.parse_function_declaration();

        assert!(result.is_ok());

        if let Ok(statement) = result {
            match statement {
                TopLevelStatement::Function(fun) => {
                    assert_eq!(fun.identifier.name, "main");
                    assert_eq!(fun.params_declaration.params.len(), 0);
                    assert!(fun.r#type.is_none());
                }
            }
        }

        let code = " fun say(): string { } ";
        let mut parser = Parser::from_code(code);

        let result = parser.parse_function_declaration();

        assert!(result.is_ok());

        if let Ok(statement) = result {
            match statement {
                TopLevelStatement::Function(fun) => {
                    assert_eq!(fun.identifier.name, "say");
                    assert_eq!(fun.params_declaration.params.len(), 0);
                    assert!(fun.r#type.is_some());
                }
            }
        }
    }

    #[test]
    fn test_params_declaration() {
        let code = " a: i32, b: string, c: char ";
        let mut parser = Parser::from_code(code);

        let result = parser.parse_params_declaration();

        assert!(result.is_ok());

        if let Ok(params) = result {
            assert_eq!(params.len(), 3);
        }
    }

    #[test]
    fn test_param_declaration() {
        let code = " a: i32 ";
        let mut parser = Parser::from_code(code);

        let result = parser.parse_param_declaration();

        assert!(result.is_ok());

        if let Ok(param) = result {
            assert_eq!(param.identifier.name, "a");
        }
    }

    #[test]
    fn test_type() {
        let code = " : i32 ";
        let mut parser = Parser::from_code(code);

        let result = parser.parse_type_optional();

        assert!(result.is_ok());

        if let Ok(param) = result {
            assert!(param.is_some());
            match param {
                Some(r#type) => assert!(matches!(r#type, SyntaxType::Simple { .. })),
                None => {}
            }
        }
    }

    #[test]
    fn test_block() {
        let code = " { { } } ";
        let mut parser = Parser::from_code(code);

        let result = parser.parse_block();
        assert!(result.is_ok());

        if let Ok(block) = result {
            assert_eq!(block.statements.len(), 1);
        }
    }

    #[test]
    fn test_statement() {
        let code = " let x = 2; ";
        let mut parser = Parser::from_code(code);

        let result = parser.parse_statement();
        assert!(result.is_ok());

        if let Ok(statement) = result {
            assert!(matches!(statement, Statement::Let(_)));
        }

        let code = " x += 3; ";
        let mut parser = Parser::from_code(code);

        let result = parser.parse_statement();
        assert!(result.is_ok());

        if let Ok(statement) = result {
            assert!(matches!(statement, Statement::Assignment(_)));
        }
    }

    #[test]
    fn test_for_expression() {
        let code = " for i in 2..0 {  } ";
        let mut parser = Parser::from_code(code);

        let result = parser.parse_for_statement();

        assert!(result.is_ok());

        if let Ok(statement) = result {
            assert!(matches!(statement, Statement::For(_)));

            match statement {
                Statement::For(r#for) => {
                    assert_eq!(r#for.identifier.name, "i");
                    assert!(matches!(r#for.expression, Expression::Range(_)))
                }
                _ => {}
            };
        }
    }

    #[test]
    fn test_do_while_statement() {
        let code = " do { x = 2; } while a; ";
        let mut parser = Parser::from_code(code);

        let result = parser.parse_do_while_statement();
        assert!(result.is_ok());

        if let Ok(statement) = result {
            assert!(matches!(statement, Statement::DoWhile(_)));
        }
    }

    #[test]
    fn test_while_statement() {
        let code = " while a { x = 2; } ";
        let mut parser = Parser::from_code(code);

        let result = parser.parse_while_statement();
        assert!(result.is_ok());

        if let Ok(statement) = result {
            assert!(matches!(statement, Statement::While(_)));
        }
    }

    #[test]
    fn test_if_statement() {
        let code = " if a == 2 { a += 2; } ";
        let mut parser = Parser::from_code(code);

        let result = parser.parse_if_statement();
        assert!(result.is_ok());

        if let Ok(statement) = result {
            assert!(matches!(statement, Statement::If(_)));

            if let Statement::If(r#if) = statement {
                assert!(r#if.r#else.is_none());
            }
        }
    }

    #[test]
    fn test_assignment_statement() {
        let code = " a += 2; ";
        let mut parser = Parser::from_code(code);

        let result = parser.parse_statement();
        assert!(result.is_ok());

        if let Ok(statement) = result {
            assert!(matches!(statement, Statement::Assignment(_)));
            if let Statement::Assignment(assignment) = statement {
                assert_eq!(assignment.operator.name, "+=");
            }
        }
    }

    #[test]
    fn test_variable_declaration_statement() {
        let code = " let x = 2; ";
        let mut parser = Parser::from_code(code);

        let result = parser.parse_variable_declaration_statement();
        assert!(result.is_ok());

        if let Ok(statement) = result {
            assert!(matches!(statement, Statement::Let(_)));

            if let Statement::Let(r#let) = statement {
                assert_eq!(r#let.identifier.name, "x");
                assert!(r#let.r#type.is_none());
            }
        }

        let code = " let x: i32 = 2; ";
        let mut parser = Parser::from_code(code);

        let result = parser.parse_variable_declaration_statement();
        assert!(result.is_ok());

        if let Ok(statement) = result {
            assert!(matches!(statement, Statement::Let(_)));

            if let Statement::Let(r#let) = statement {
                assert_eq!(r#let.identifier.name, "x");
                assert!(r#let.r#type.is_some());
            }
        }
    }

    #[test]
    fn test_return_statement() {
        let code = " return; ";
        let mut parser = Parser::from_code(code);

        let result = parser.parse_return_statement();
        assert!(result.is_ok());

        if let Ok(statement) = result {
            assert!(matches!(statement, Statement::Return(_)));
            if let Statement::Return(r#return) = statement {
                assert!(r#return.expression.is_none());
            }
        }

        let code = " return 2; ";
        let mut parser = Parser::from_code(code);

        let result = parser.parse_return_statement();
        assert!(result.is_ok());

        if let Ok(statement) = result {
            assert!(matches!(statement, Statement::Return(_)));
            if let Statement::Return(r#return) = statement {
                assert!(r#return.expression.is_some());
            }
        }
    }

    #[test]
    fn test_expression() {
        let code = " (1 + 2) * 3 ";
        let mut parser = Parser::from_code(code);

        let result = parser.parse_expression(0);
        assert!(result.is_ok());

        if let Ok(expression) = result {
            assert!(matches!(expression, Expression::Binary(_)));
        }

        let code = " a ";
        let mut parser = Parser::from_code(code);

        let result = parser.parse_expression(0);
        assert!(result.is_ok());

        if let Ok(expression) = result {
            assert!(matches!(expression, Expression::Identifier(_, _)));
        }

        let code = " 0..=3 ";
        let mut parser = Parser::from_code(code);

        let result = parser.parse_expression(0);
        assert!(result.is_ok());

        if let Ok(expression) = result {
            assert!(matches!(expression, Expression::Range(_)));
        }
    }
}
