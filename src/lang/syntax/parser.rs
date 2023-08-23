use std::collections::VecDeque;

use super::compilation_unit::CompilationUnit;
use super::expressions::{BinaryOperator, Expression, Parenthesis, UnaryOperator};
use super::lexer::{Kind, Lexer, Token};
use super::statements::{
    AssignmentOperator, Block, Brace, Colon, FunKeyword, Function, Identifier, Let, LetKeyword,
    Return, ReturnKeyword, Semicolon, Statement, TopLevelStatement,
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

        if bad_token.is_some() {
            return Err("Bad token".to_string());
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
        let fun = self.next_token();
        let identifier = self.next_token();

        if identifier.kind != Kind::Identifier {
            return Err("identifier expected".to_string());
        }

        let open_parenthesis = self.next_token();

        if open_parenthesis.kind != Kind::OpenParenthesis {
            return Err("Parenthesis expected".to_string());
        }

        let close_parenthesis = self.next_token();

        if close_parenthesis.kind != Kind::CloseParenthesis {
            return Err("Parenthesis expected".to_string());
        }

        let next = self.current_token();

        match next.kind {
            Kind::Colon => {
                let colon = self.next_token();
                let t = self.next_token();

                if t.kind != Kind::Identifier {
                    return Err("Type expected".to_string());
                }

                let block = self.parse_block_top_level_statement()?;

                Ok(TopLevelStatement::Function(Function::Typed(
                    FunKeyword(fun),
                    Identifier(identifier),
                    Parenthesis(open_parenthesis),
                    Parenthesis(close_parenthesis),
                    Colon(colon),
                    Identifier(t),
                    block,
                )))
            }
            Kind::OpenBraces => {
                let block = self.parse_block_top_level_statement()?;

                Ok(TopLevelStatement::Function(Function::Untyped(
                    FunKeyword(fun),
                    Identifier(identifier),
                    Parenthesis(open_parenthesis),
                    Parenthesis(close_parenthesis),
                    block,
                )))
            }
            _ => Err("Type or block expected".to_string()),
        }
    }

    fn parse_statement(&mut self) -> Result<Statement, String> {
        match self.current_token().kind {
            Kind::OpenBraces => self.parse_block_statement(),
            Kind::Identifier => self.parse_assignment(),
            Kind::Let => Ok(Statement::Let(self.parse_variable_declaration_statement()?)),
            Kind::Return => {
                let r = self.next_token(); // return

                match self.current_token().kind {
                    Kind::Semicolon => {
                        let semicolon = self.next_token();

                        if semicolon.kind != Kind::Semicolon {
                            return Err("Semicolon expected".to_string());
                        }

                        Ok(Statement::Return(Return::WithoutExpression(
                            ReturnKeyword(r),
                            Semicolon(semicolon),
                        )))
                    }
                    _ => {
                        let expression = self.parse_expression(0)?;
                        let semicolon = self.next_token();

                        if semicolon.kind != Kind::Semicolon {
                            return Err("Semicolon expected".to_string());
                        }

                        Ok(Statement::Return(Return::WithExpression(
                            ReturnKeyword(r),
                            expression,
                            Semicolon(semicolon),
                        )))
                    }
                }
            }
            _ => Err("Statement expected".to_string()),
        }
    }

    fn parse_block_top_level_statement(&mut self) -> Result<Block, String> {
        let token = self.next_token();

        let open_brace = Brace(token);
        let mut statements: Vec<Statement> = vec![];

        while self.current_token().kind != Kind::CloseBraces {
            let statement = self.parse_statement()?;
            statements.push(statement);
        }

        let token = self.next_token();

        if token.kind != Kind::CloseBraces {
            return Err("Block end expected".to_string());
        }

        let close_brace = Brace(token);

        Ok(Block(open_brace, statements, close_brace))
    }

    fn parse_block_statement(&mut self) -> Result<Statement, String> {
        let token = self.next_token();

        let open_brace = Brace(token);
        let mut statements: Vec<Statement> = vec![];

        while self.current_token().kind != Kind::CloseBraces {
            let statement = self.parse_statement()?;
            statements.push(statement);
        }

        let token = self.next_token();

        if token.kind != Kind::CloseBraces {
            return Err("Block end expected".to_string());
        }

        let close_brace = Brace(token);

        Ok(Statement::Block(Block(open_brace, statements, close_brace)))
    }

    fn parse_assignment(&mut self) -> Result<Statement, String> {
        let identifier = self.next_token();

        let operator = self.next_token();

        if operator.kind != Kind::Equals
            && operator.kind != Kind::AmpersandEquals
            && operator.kind != Kind::PipeEquals
            && operator.kind != Kind::PlusEquals
            && operator.kind != Kind::MinusEquals
            && operator.kind != Kind::StarEquals
            && operator.kind != Kind::SlashEquals
            && operator.kind != Kind::ModEquals
            && operator.kind != Kind::CircumflexEquals
        {
            return Err("Assignment operator expected".to_string());
        }

        let expression = self.parse_expression(0)?;

        let semicolon_token = self.next_token();

        if semicolon_token.kind != Kind::Semicolon {
            return Err("Semicolon expected".to_string());
        }

        Ok(Statement::Assignment(
            Identifier(identifier),
            AssignmentOperator(operator),
            expression,
            Semicolon(semicolon_token),
        ))
    }

    fn parse_variable_declaration_statement(&mut self) -> Result<Let, String> {
        // REVIEW: Should we return the statement, instead of the Let?

        let let_token = self.next_token();

        let identifier_token = self.next_token();
        if identifier_token.kind != Kind::Identifier {
            return Err("Identifier expected".to_string());
        }

        let next = self.current_token();

        match next.kind {
            Kind::Equals => {
                let assignment_token = self.next_token();
                if assignment_token.kind != Kind::Equals {
                    return Err("Assignment operator expected".to_string());
                }

                let expression = self.parse_expression(0)?;

                let semicolon_token = self.next_token();
                if semicolon_token.kind != Kind::Semicolon {
                    return Err("Semicolon expected".to_string());
                }

                Ok(Let::UntypedWithValue(
                    LetKeyword(let_token),
                    Identifier(identifier_token),
                    AssignmentOperator(assignment_token),
                    expression,
                    Semicolon(semicolon_token),
                ))
            }
            Kind::Colon => {
                let colon_token = self.next_token();
                let type_token = self.next_token();

                if type_token.kind != Kind::Identifier {
                    return Err("Type expected".to_string());
                }

                let next = self.current_token();

                match next.kind {
                    Kind::Semicolon => {
                        let semicolon_token = self.next_token();
                        Ok(Let::TypedWithoutValue(
                            LetKeyword(let_token),
                            Identifier(identifier_token),
                            Colon(colon_token),
                            Identifier(type_token),
                            Semicolon(semicolon_token),
                        ))
                    }
                    Kind::Equals => {
                        let equals_token = self.next_token();
                        let expression = self.parse_expression(0)?;

                        let semicolon_token = self.next_token();
                        if semicolon_token.kind != Kind::Semicolon {
                            return Err("Semicolon expected".to_string());
                        }

                        Ok(Let::TypedWithValue(
                            LetKeyword(let_token),
                            Identifier(identifier_token),
                            Colon(colon_token),
                            Identifier(type_token),
                            AssignmentOperator(equals_token),
                            expression,
                            Semicolon(semicolon_token),
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

    fn parse_factor(&mut self) -> Result<Expression, String> {
        let token = self.next_token();

        match token.kind {
            Kind::Identifier => Ok(Expression::Identifier(token)),
            Kind::Number | Kind::Boolean => Ok(Expression::Literal(token)),
            Kind::OpenParenthesis => {
                let open_parenthesis = Parenthesis(token);
                let expression = self.parse_expression(0)?;

                let token = self.next_token();

                if token.kind != Kind::CloseParenthesis {
                    return Err("Expected close parenthesis".to_string());
                }

                let close_parenthesis = Parenthesis(token);
                Ok(Expression::Parenthesized(
                    open_parenthesis,
                    Box::new(expression),
                    close_parenthesis,
                ))
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
