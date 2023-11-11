use std::collections::VecDeque;

use super::compilation_unit::CompilationUnit;
use super::expressions::{BinaryOperator, Expression, UnaryOperator};

use super::lexer::kind::Kind;
use super::lexer::lexer::Lexer;
use super::lexer::token::Token;
use super::statements::{
    AssignmentOperator, Block, ElseStatement, Function, Identifier, Let, ParamDeclaration, Params,
    ParamsDeclaration, Statement, TopLevelStatement,
};

pub struct Parser {
    tokens: VecDeque<Token>,
}

impl Parser {
    pub fn new(text: &str) -> Self {
        let mut tokens = VecDeque::new();
        let mut lexer = Lexer::new(text);

        loop {
            let token = lexer.next();

            match token.kind {
                Kind::WhiteSpace => continue,
                Kind::EndOfFile => {
                    tokens.push_back(token);
                    break;
                }
                _ => tokens.push_back(token),
            }
        }

        Self { tokens }
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

    fn current_token(&self) -> &Token {
        self.tokens.get(0).unwrap()
    }

    fn use_token(&mut self, kind: Kind) -> Result<Token, String> {
        let token = self.next_token();

        if token.kind != kind {
            Err(format!(
                "Expected a token of type '{}' at Line {} and Column {}",
                kind, token.position.line, token.position.column
            ))
        } else {
            Ok(token)
        }
    }

    fn next_token(&mut self) -> Token {
        self.tokens.pop_front().unwrap()
    }

    fn parse_top_level_statement(&mut self) -> Result<TopLevelStatement, String> {
        match self.current_token().kind {
            Kind::Fun => self.parse_function_declaration(),
            _ => Err("Top-level statement expected".to_string()),
        }
    }

    fn parse_function_declaration(&mut self) -> Result<TopLevelStatement, String> {
        self.next_token(); // consumes the fun keyword

        let identifier = self.use_token(Kind::Identifier)?;
        self.use_token(Kind::OpenParenthesis)?;

        let mut params: Vec<ParamDeclaration> = vec![];
        if self.current_token().kind == Kind::Identifier {
            let param = self.parse_param_declaration()?;
            params.push(param);

            while self.current_token().kind == Kind::Comma {
                self.next_token();
                let param = self.parse_param_declaration()?;
                params.push(param);
            }
        }

        self.use_token(Kind::CloseParenthesis)?;

        let next = self.current_token();

        match next.kind {
            Kind::Colon => {
                self.use_token(Kind::Colon)?;

                let t_token = self.use_token(Kind::Identifier)?;

                let block = self.parse_block()?;

                Ok(TopLevelStatement::Function(Function(
                    Identifier(identifier),
                    ParamsDeclaration(params),
                    Some(Identifier(t_token)),
                    block,
                )))
            }
            Kind::OpenBraces => {
                let block = self.parse_block()?;

                Ok(TopLevelStatement::Function(Function(
                    Identifier(identifier),
                    ParamsDeclaration(params),
                    None,
                    block,
                )))
            }
            _ => Err("Type or block expected".to_string()),
        }
    }

    fn parse_param_declaration(&mut self) -> Result<ParamDeclaration, String> {
        let param_name = self.use_token(Kind::Identifier)?;
        self.use_token(Kind::Colon)?;
        let t_token = self.use_token(Kind::Identifier)?;

        Ok(ParamDeclaration(
            Identifier(param_name),
            Identifier(t_token),
        ))
    }

    fn parse_statement(&mut self) -> Result<Statement, String> {
        match self.current_token().kind {
            Kind::While => self.parse_while_statement(),
            Kind::If => self.parse_if_statement(),
            Kind::OpenBraces => Ok(Statement::Block(self.parse_block()?)),
            Kind::Identifier => {
                let identifier = self.next_token();
                match self.current_token().kind {
                    Kind::OpenParenthesis => self.parse_function_call(identifier),
                    Kind::Equals
                    | Kind::AmpersandEquals
                    | Kind::PipeEquals
                    | Kind::PlusEquals
                    | Kind::MinusEquals
                    | Kind::StarEquals
                    | Kind::SlashEquals
                    | Kind::ModEquals
                    | Kind::CircumflexEquals => self.parse_assignment(identifier),
                    _ => Err("Assignment operator or function call expected".to_string()),
                }
            }
            Kind::Let => Ok(Statement::Let(self.parse_variable_declaration_statement()?)),
            Kind::Return => self.parse_return_statement(),
            _ => Err("Statement expected".to_string()),
        }
    }

    fn parse_return_statement(&mut self) -> Result<Statement, String> {
        self.next_token(); // consumes the return keyword

        match self.current_token().kind {
            Kind::Semicolon => {
                self.use_token(Kind::Semicolon)?;
                Ok(Statement::Return(None))
            }
            _ => {
                let expression = self.parse_expression(0)?;
                self.use_token(Kind::Semicolon)?;
                Ok(Statement::Return(Some(expression)))
            }
        }
    }

    fn parse_block(&mut self) -> Result<Block, String> {
        self.next_token(); // consumes the open brace

        let mut statements: Vec<Statement> = vec![];
        while self.current_token().kind != Kind::CloseBraces {
            let statement = self.parse_statement()?;
            statements.push(statement);
        }

        self.use_token(Kind::CloseBraces)?;

        Ok(Block(statements))
    }

    fn parse_while_statement(&mut self) -> Result<Statement, String> {
        self.next_token(); // consumes the while keyword

        let expression = self.parse_expression(0)?;
        let statement = self.parse_statement()?;

        Ok(Statement::While(expression, Box::new(statement)))
    }

    fn parse_if_statement(&mut self) -> Result<Statement, String> {
        self.next_token(); // consumes the if keyword

        let expression = self.parse_expression(0)?;
        let statement = self.parse_statement()?;

        if self.current_token().kind == Kind::Else {
            self.next_token();

            let else_statement = self.parse_statement()?;
            Ok(Statement::If(
                expression,
                Box::new(statement),
                Some(ElseStatement(Box::new(else_statement))),
            ))
        } else {
            Ok(Statement::If(expression, Box::new(statement), None))
        }
    }

    fn parse_assignment(&mut self, identifier: Token) -> Result<Statement, String> {
        let operator = self.next_token();
        let expression = self.parse_expression(0)?;

        self.use_token(Kind::Semicolon)?;

        Ok(Statement::Assignment(
            Identifier(identifier),
            AssignmentOperator(operator),
            expression,
        ))
    }

    fn parse_function_call(&mut self, identifier: Token) -> Result<Statement, String> {
        self.next_token(); // consumes open parenthesis

        let params = self.parse_params()?;

        self.next_token(); // consumes close parenthesis
        self.next_token(); // consumes semicolon

        Ok(Statement::FunctionCall(Identifier(identifier), params))
    }

    fn parse_variable_declaration_statement(&mut self) -> Result<Let, String> {
        // REVIEW: Should we return the statement, instead of the Let?

        self.next_token(); // consumes the let keyword

        let identifier_token = self.use_token(Kind::Identifier)?;

        match self.current_token().kind {
            Kind::Equals => {
                let assignment_token = self.use_token(Kind::Equals)?;
                let expression = self.parse_expression(0)?;
                self.use_token(Kind::Semicolon)?;

                Ok(Let::WithValue(
                    Identifier(identifier_token),
                    None,
                    AssignmentOperator(assignment_token),
                    expression,
                ))
            }
            Kind::Colon => {
                self.next_token(); // consumes the colon token

                let type_token = self.use_token(Kind::Identifier)?;

                match self.current_token().kind {
                    Kind::Semicolon => {
                        self.use_token(Kind::Semicolon)?;

                        Ok(Let::WithoutValue(
                            Identifier(identifier_token),
                            Identifier(type_token),
                        ))
                    }
                    Kind::Equals => {
                        let equals_token = self.next_token(); // consumes the equals token
                        let expression = self.parse_expression(0)?;
                        self.use_token(Kind::Semicolon)?;

                        Ok(Let::WithValue(
                            Identifier(identifier_token),
                            Some(Identifier(type_token)),
                            AssignmentOperator(equals_token),
                            expression,
                        ))
                    }
                    _ => Err("Semicolon or assignment operator expected".to_string()),
                }
            }
            _ => Err("Assignment operator or colon expected".to_string()),
        }
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
            Kind::Number | Kind::Boolean => Ok(Expression::Literal(token)),
            Kind::Identifier => {
                let identifier = token;

                match self.current_token().kind {
                    Kind::OpenParenthesis => {
                        self.next_token(); // consumes open parenthesis
                        let params = self.parse_params()?;
                        self.next_token(); // consumes close parenthesis

                        Ok(Expression::FunctionCall(Identifier(identifier), params))
                    }
                    _ => Ok(Expression::Identifier(identifier)),
                }
            }
            Kind::OpenParenthesis => {
                let expression = self.parse_expression(0)?;

                self.use_token(Kind::CloseParenthesis)?;

                Ok(Expression::Parenthesized(Box::new(expression)))
            }
            _ => Err("Expression expected".to_string()),
        }
    }
}

fn get_unary_operator_precedence(kind: Kind) -> u32 {
    match kind {
        Kind::Plus | Kind::Minus | Kind::Exclamation | Kind::Tilde => 10,
        _ => 0,
    }
}

fn get_binary_operator_precedence(kind: Kind) -> u32 {
    match kind {
        Kind::Slash | Kind::Star | Kind::Mod => 9,
        Kind::Minus | Kind::Plus => 8,
        Kind::GreaterThan | Kind::GreaterThanEquals | Kind::LessThan | Kind::LessThanEquals => 7,
        Kind::Equals | Kind::EqualsEquals => 6,
        Kind::Ampersand => 5,
        Kind::Circumflex => 4,
        Kind::Pipe => 3,
        Kind::AmpersandAmpersand => 2,
        Kind::PipePipe => 1,
        _ => 0,
    }
}