use std::collections::VecDeque;

use crate::lang::syntax::lexer::kind::Kind;
use crate::lang::syntax::lexer::lexer::Lexer;
use crate::lang::syntax::lexer::token::Token;

use super::compilation_unit::CompilationUnit;
use super::expressions::{BinaryOperator, Expression, UnaryOperator};

use super::statements::{
    AssignmentOperator, Block, Const, ElseStatement, Identifier, Let, ParamDeclaration, Params,
    ParamsDeclaration, Statement, TopLevelStatement,
};

pub struct Parser {
    tokens: VecDeque<Token>,
}

impl Parser {
    pub fn new(text: &str) -> Self {
        let mut tokens = VecDeque::new();
        let mut lexer = Lexer::new(text);

        let mut token = lexer.next();

        while token.kind != Kind::EndOfFile {
            if token.kind != Kind::WhiteSpace {
                tokens.push_back(token);
            }
            token = lexer.next();
        }
        tokens.push_back(token);

        Self { tokens }
    }

    fn current_token(&self) -> &Token {
        self.tokens.get(0).unwrap()
    }

    fn use_token(&mut self, kinds: &[Kind]) -> Result<Token, String> {
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
        let bad_token = self.tokens.iter().find(|token| token.kind == Kind::Bad);

        if let Some(bad_token) = bad_token {
            return Err(format!(
                "Invalid token found at Line {} and Column {}",
                bad_token.position.line, bad_token.position.column
            ));
        }

        let mut statements: Vec<TopLevelStatement> = vec![];

        let mut token = self.current_token();
        while token.kind != Kind::EndOfFile {
            let statement = self.parse_top_level_statement()?;
            statements.push(statement);
            token = self.current_token();
        }

        return Ok(CompilationUnit(statements));
    }

    /// Parses a top-level statement.
    ///
    /// # Returns
    /// - `Ok(TopLevelStatement)`: Parsed top-level statement.
    /// - `Err(String)`: Error message if parsing fails.
    fn parse_top_level_statement(&mut self) -> Result<TopLevelStatement, String> {
        let token = self.current_token();
        match token.kind {
            Kind::Fun => self.parse_function_declaration(),
            _ => Err(format!("Top-level statement expected")),
        }
    }

    /// Parses a function declaration in the format: `fun id(params) { ... }`.
    ///
    /// # Returns
    /// - `Ok(TopLevelStatement)`: Parsed function declaration as a top-level statement.
    /// - `Err(String)`: Error message if parsing fails.
    fn parse_function_declaration(&mut self) -> Result<TopLevelStatement, String> {
        self.use_token(&[Kind::Fun])?;

        let id_token = self.use_token(&[Kind::Identifier])?;
        let id = Identifier(id_token);

        self.use_token(&[Kind::OpenParenthesis])?;

        let params: Vec<ParamDeclaration> = self.parse_params_declaration()?;

        self.use_token(&[Kind::CloseParenthesis])?;

        let next = self.current_token();

        match next.kind {
            Kind::Colon => {
                self.use_token(&[Kind::Colon])?;

                let type_id_token = self.use_token(&[Kind::Identifier])?;
                let type_id = Identifier(type_id_token);
                let block = self.parse_block()?;

                Ok(TopLevelStatement::Function(
                    id,
                    ParamsDeclaration(params),
                    Some(type_id),
                    block,
                ))
            }
            Kind::OpenBraces => {
                let block = self.parse_block()?;

                Ok(TopLevelStatement::Function(
                    id,
                    ParamsDeclaration(params),
                    None,
                    block,
                ))
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
        let mut params: Vec<ParamDeclaration> = vec![];

        if self.current_token().kind == Kind::Identifier {
            let param = self.parse_param_declaration()?;
            params.push(param);

            while self.current_token().kind == Kind::Comma {
                self.use_token(&[Kind::Comma])?;
                let param = self.parse_param_declaration()?;
                params.push(param);
            }
        }

        Ok(params)
    }

    /// Parses a parameter declaration in the form: `id : type_id`.
    ///
    /// # Returns
    /// - `Ok(ParamDeclaration)`: Parsed parameter declaration.
    /// - `Err(String)`: Parsing error message.
    fn parse_param_declaration(&mut self) -> Result<ParamDeclaration, String> {
        let param_name_token = self.use_token(&[Kind::Identifier])?;

        self.use_token(&[Kind::Colon])?;

        let type_id_token = self.use_token(&[Kind::Identifier])?;

        Ok(ParamDeclaration(
            Identifier(param_name_token),
            Identifier(type_id_token),
        ))
    }

    fn parse_statement(&mut self) -> Result<Statement, String> {
        match self.current_token().kind {
            Kind::For => self.parse_for_statement(),
            Kind::Do => self.parse_do_while_statement(),
            Kind::While => self.parse_while_statement(),
            Kind::If => self.parse_if_statement(),
            Kind::OpenBraces => self.parse_block().map(|block| Statement::Block(block)),
            Kind::Identifier => {
                let identifier = self.next_token();
                match self.current_token().kind {
                    Kind::OpenParenthesis => self.parse_function_call_statement(identifier),
                    Kind::Equals
                    | Kind::AmpersandEquals
                    | Kind::PipeEquals
                    | Kind::PlusEquals
                    | Kind::MinusEquals
                    | Kind::StarEquals
                    | Kind::SlashEquals
                    | Kind::ModEquals
                    | Kind::CircumflexEquals => self.parse_assignment(identifier),
                    _ => Err(format!("Assignment operator or function call expected",)),
                }
            }
            Kind::Let => Ok(self.parse_variable_declaration_statement()?),
            Kind::Const => Ok(self.parse_constant_declaration_statement()?),
            Kind::Return => self.parse_return_statement(),
            _ => Err("Statement expected".to_string()),
        }
    }

    /// Parses a 'return' statement.
    ///
    /// # Returns
    /// - `Ok(Statement)`: Parsed 'return' statement.
    /// - `Err(String)`: Error message if parsing fails.
    fn parse_return_statement(&mut self) -> Result<Statement, String> {
        self.use_token(&[Kind::Return])?;

        match self.current_token().kind {
            Kind::Semicolon => {
                self.use_token(&[Kind::Semicolon])?;
                Ok(Statement::Return(None))
            }
            _ => {
                let expression = self.parse_expression(0)?;
                self.use_token(&[Kind::Semicolon])?;
                Ok(Statement::Return(Some(expression)))
            }
        }
    }

    /// Parses a block of statements enclosed within braces `{}`.
    ///
    /// # Returns
    /// - `Ok(Block)`: Parsed block of statements.
    /// - `Err(String)`: Error message if parsing fails.
    fn parse_block(&mut self) -> Result<Block, String> {
        self.use_token(&[Kind::OpenBraces])?;

        let mut statements: Vec<Statement> = vec![];
        while self.current_token().kind != Kind::CloseBraces {
            let statement = self.parse_statement()?;
            statements.push(statement);
        }

        self.use_token(&[Kind::CloseBraces])?;

        Ok(Block(statements))
    }

    /// Parses a 'while' loop statement in the format: `while condition { statement }`.
    ///
    /// # Returns
    /// - `Ok(Statement)`: Parsed 'while' loop statement.
    /// - `Err(String)`: Error message if parsing fails.
    fn parse_while_statement(&mut self) -> Result<Statement, String> {
        self.use_token(&[Kind::While])?;

        let expression = self.parse_expression(0)?;
        let statement = self.parse_statement()?;

        Ok(Statement::While(expression, Box::new(statement)))
    }

    /// Parses a 'while' loop statement in the format: `while condition { statement }`.
    ///
    /// # Returns
    /// - `Ok(Statement)`: Parsed 'while' loop statement.
    /// - `Err(String)`: Error message if parsing fails.
    fn parse_do_while_statement(&mut self) -> Result<Statement, String> {
        self.use_token(&[Kind::Do])?;
        let statement = self.parse_statement()?;
        self.use_token(&[Kind::While])?;
        let expression = self.parse_expression(0)?;

        Ok(Statement::DoWhile(Box::new(statement), expression))
    }

    /// Parses a 'for' loop statement in the format: `for condition in expression { statement }`.
    ///
    /// # Returns
    /// - `Ok(Statement)`: Parsed 'for' loop statement.
    /// - `Err(String)`: Error message if parsing fails.
    fn parse_for_statement(&mut self) -> Result<Statement, String> {
        self.use_token(&[Kind::For])?;
        let id = self.use_token(&[Kind::Identifier])?;
        self.use_token(&[Kind::In])?;
        let expression = self.parse_expression(0)?;
        let statement = self.parse_statement()?;

        Ok(Statement::For(
            Identifier(id),
            expression,
            Box::new(statement),
        ))
    }

    /// Parses an 'if' statement in the format: `if condition { statement } [else { else_statement }]`.
    ///
    /// Note that else blocks are optional.
    ///
    /// # Returns
    /// - `Ok(Statement)`: Parsed 'if' statement.
    /// - `Err(String)`: Error message if parsing fails.
    fn parse_if_statement(&mut self) -> Result<Statement, String> {
        self.use_token(&[Kind::If])?;

        let expression = self.parse_expression(0)?;
        let statement = self.parse_statement()?;

        if self.current_token().kind != Kind::Else {
            return Ok(Statement::If(expression, Box::new(statement), None));
        }

        self.use_token(&[Kind::Else])?;

        let else_statement = self.parse_statement()?;

        Ok(Statement::If(
            expression,
            Box::new(statement),
            Some(ElseStatement(Box::new(else_statement))),
        ))
    }

    /// Parses a function call statement given an identifier.
    ///
    /// # Arguments
    /// - `identifier`: Token representing the function identifier.
    ///
    /// # Returns
    /// - `Ok(Statement)`: Parsed function call statement.
    /// - `Err(String)`: Error message if parsing fails.
    fn parse_function_call_statement(&mut self, identifier: Token) -> Result<Statement, String> {
        self.use_token(&[Kind::OpenParenthesis])?;
        let params = self.parse_params()?;
        self.use_token(&[Kind::CloseParenthesis])?;
        self.use_token(&[Kind::Semicolon])?;

        Ok(Statement::FunctionCall(Identifier(identifier), params))
    }

    /// Parses a function call expression given an identifier.
    ///
    /// # Arguments
    /// - `identifier`: Token representing the function identifier.
    ///
    /// # Returns
    /// - `Ok(Statement)`: Parsed function call expression.
    /// - `Err(String)`: Error message if parsing fails.
    fn parse_function_call_expression(&mut self, identifier: Token) -> Result<Expression, String> {
        self.use_token(&[Kind::OpenParenthesis])?;
        let params = self.parse_params()?;
        self.use_token(&[Kind::CloseParenthesis])?;

        Ok(Expression::FunctionCall(Identifier(identifier), params))
    }

    /// Parses a function call statement given an identifier.
    ///
    /// # Arguments
    /// - `identifier`: Token representing the function identifier.
    ///
    /// # Returns
    /// - `Ok(Statement)`: Parsed function call statement.
    /// - `Err(String)`: Error message if parsing fails.
    fn parse_assignment(&mut self, identifier: Token) -> Result<Statement, String> {
        let operator = self.use_token(&[
            Kind::Equals,
            Kind::AmpersandEquals,
            Kind::PipeEquals,
            Kind::CircumflexEquals,
            Kind::TildeEquals,
            Kind::PlusEquals,
            Kind::MinusEquals,
            Kind::StarEquals,
            Kind::SlashEquals,
            Kind::ModEquals,
        ])?;

        let expression = self.parse_expression(0)?;

        self.use_token(&[Kind::Semicolon])?;

        Ok(Statement::Assignment(
            Identifier(identifier),
            AssignmentOperator(operator),
            expression,
        ))
    }

    fn parse_variable_declaration_statement(&mut self) -> Result<Statement, String> {
        self.use_token(&[Kind::Let])?;
        let identifier_token = self.use_token(&[Kind::Identifier])?;

        if let Kind::Equals = self.current_token().kind {
            let assignment_token = self.use_token(&[Kind::Equals])?;
            let expression = self.parse_expression(0)?;
            self.use_token(&[Kind::Semicolon])?;

            return Ok(Statement::Let(Let::WithValue(
                Identifier(identifier_token),
                None,
                AssignmentOperator(assignment_token),
                expression,
            )));
        }

        if let Kind::Colon = self.current_token().kind {
            self.use_token(&[Kind::Colon])?;
            let type_id_token = self.use_token(&[Kind::Identifier])?;

            if let Kind::Semicolon = self.current_token().kind {
                self.use_token(&[Kind::Semicolon])?;

                return Ok(Statement::Let(Let::WithoutValue(
                    Identifier(identifier_token),
                    Identifier(type_id_token),
                )));
            }

            if let Kind::Equals = self.current_token().kind {
                let equals_token = self.use_token(&[Kind::Equals])?;
                let expression = self.parse_expression(0)?;
                self.use_token(&[Kind::Semicolon])?;

                return Ok(Statement::Let(Let::WithValue(
                    Identifier(identifier_token),
                    Some(Identifier(type_id_token)),
                    AssignmentOperator(equals_token),
                    expression,
                )));
            }

            return Err("Semicolon or assignment operator expected".to_string());
        }

        Err("Assignment operator or colon expected".to_string())
    }

    fn parse_constant_declaration_statement(&mut self) -> Result<Statement, String> {
        self.use_token(&[Kind::Const])?;
        let identifier_token = self.use_token(&[Kind::Identifier])?;

        if let Kind::Equals = self.current_token().kind {
            let assignment_token = self.use_token(&[Kind::Equals])?;
            let expression = self.parse_expression(0)?;
            self.use_token(&[Kind::Semicolon])?;

            return Ok(Statement::Const(Const::WithValue(
                Identifier(identifier_token),
                None,
                AssignmentOperator(assignment_token),
                expression,
            )));
        }

        if let Kind::Colon = self.current_token().kind {
            self.use_token(&[Kind::Colon])?;
            let type_id_token = self.use_token(&[Kind::Identifier])?;

            if let Kind::Semicolon = self.current_token().kind {
                self.use_token(&[Kind::Semicolon])?;

                return Ok(Statement::Const(Const::WithoutValue(
                    Identifier(identifier_token),
                    Identifier(type_id_token),
                )));
            }

            if let Kind::Equals = self.current_token().kind {
                let equals_token = self.use_token(&[Kind::Equals])?;
                let expression = self.parse_expression(0)?;
                self.use_token(&[Kind::Semicolon])?;

                return Ok(Statement::Const(Const::WithValue(
                    Identifier(identifier_token),
                    Some(Identifier(type_id_token)),
                    AssignmentOperator(equals_token),
                    expression,
                )));
            }

            return Err("Semicolon or assignment operator expected".to_string());
        }

        Err("Assignment operator or colon expected".to_string())
    }

    fn parse_expression(&mut self, parent_precedence: u32) -> Result<Expression, String> {
        let mut left: Expression;
        let token = self.current_token();

        let unary_precedence = get_unary_operator_precedence(token.kind);
        if unary_precedence != 0 && unary_precedence >= parent_precedence {
            let operator_token = self.next_token();
            left = Expression::Unary(
                UnaryOperator(operator_token),
                Box::new(self.parse_expression(unary_precedence)?),
            );
        } else {
            left = self.parse_factor()?;
        }

        let token = self.current_token();
        let mut precedence = get_binary_operator_precedence(token.kind);

        while precedence != 0 && precedence > parent_precedence {
            let operator_token = self.next_token();
            let operator = BinaryOperator(operator_token);
            let right = self.parse_expression(precedence)?;

            left = Expression::Binary(Box::new(left), operator, Box::new(right));

            precedence = get_binary_operator_precedence(self.current_token().kind);
        }

        Ok(left)
    }

    fn parse_params(&mut self) -> Result<Params, String> {
        if self.current_token().kind == Kind::CloseParenthesis {
            return Ok(Params(vec![]));
        }

        let mut expressions: Vec<Expression> = vec![];

        if self.current_token().kind != Kind::CloseParenthesis {
            loop {
                let expression = self.parse_expression(0)?;
                expressions.push(expression);

                if self.current_token().kind == Kind::Comma {
                    self.next_token(); // consumes the comma
                } else {
                    break;
                }
            }
        }

        Ok(Params(expressions))
    }

    fn parse_factor(&mut self) -> Result<Expression, String> {
        let token = self.next_token();

        match token.kind {
            Kind::Number | Kind::Boolean | Kind::String | Kind::Char => {
                Ok(Expression::Literal(token))
            }
            Kind::Identifier => {
                let identifier = token;

                match self.current_token().kind {
                    Kind::OpenParenthesis => self.parse_function_call_expression(identifier),
                    _ => Ok(Expression::Identifier(identifier)),
                }
            }
            Kind::OpenParenthesis => {
                let expression = self.parse_expression(0)?;
                self.use_token(&[Kind::CloseParenthesis])?;
                Ok(Expression::Parenthesized(Box::new(expression)))
            }
            _ => Err("Expression expected".to_string()),
        }
    }
}

fn get_unary_operator_precedence(kind: Kind) -> u32 {
    match kind {
        Kind::Plus | Kind::Minus | Kind::Exclamation | Kind::Tilde => 11,
        _ => 0,
    }
}

fn get_binary_operator_precedence(kind: Kind) -> u32 {
    match kind {
        Kind::Slash | Kind::Star | Kind::Mod => 10,
        Kind::Minus | Kind::Plus => 9,
        Kind::GreaterThan | Kind::GreaterThanEquals | Kind::LessThan | Kind::LessThanEquals => 8,
        Kind::Equals | Kind::EqualsEquals => 7,
        Kind::Ampersand => 6,
        Kind::Circumflex => 5,
        Kind::Pipe => 4,
        Kind::AmpersandAmpersand => 3,
        Kind::PipePipe => 2,
        Kind::DotDot | Kind::DotDotEquals => 1,
        _ => 0,
    }
}

#[cfg(test)]
mod tests {
    use crate::lang::syntax::{
        lexer::kind::Kind,
        parser::{
            expressions::{BinaryOperator, Expression},
            statements::Statement,
        },
    };

    use super::Parser;

    #[test]
    fn test_for_expression() {
        let code = "for i in 2..0 {  }";
        let mut parser = Parser::new(code);

        let statement = parser.parse_for_statement();
        assert!(statement.is_ok());

        if let Ok(for_statement) = statement {
            assert!(matches!(for_statement, Statement::For(_, _, _)));

            if let Statement::For(id, binary_expression, _) = for_statement {
                assert_eq!(id.0.value.unwrap(), "i");

                if let Expression::Binary(_, binary_operator, _) = binary_expression {
                    assert_eq!(binary_operator.0.kind, Kind::DotDot);
                }
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
            assert!(matches!(binary_expression, Expression::Binary(_, _, _)));

            if let Expression::Binary(left_expression, binary_operator, _) = binary_expression {
                assert!(matches!(left_expression.as_ref(), Expression::Literal(_)));
                assert!(matches!(binary_operator, BinaryOperator(_)));
            }
        }
    }
}
