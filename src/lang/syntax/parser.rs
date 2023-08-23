use std::collections::VecDeque;

use crate::lang::syntax::statements::{AssignmentOperator, Brace};

use super::expressions::{BinaryOperator, Expression, Parenthesis, UnaryOperator};
use super::lexer::{Kind, Lexer, Token};
use super::statements::{Colon, Identifier, Let, LetKeyword, Semicolon, Statement};

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

    pub fn parse(&mut self) -> Result<Statement, String> {
        let bad_token = self.tokens.iter().find(|token| token.kind == Kind::Bad);

        if bad_token.is_some() {
            return Err("Bad token".to_string());
        }

        let statement = self.parse_statement()?;
        return Ok(statement);
    }

    fn current_token(&self) -> &Token {
        self.tokens.get(0).unwrap()
    }

    fn next_token(&mut self) -> Token {
        self.tokens.pop_front().unwrap()
    }

    fn parse_statement(&mut self) -> Result<Statement, String> {
        match self.current_token().kind {
            Kind::OpenBraces => self.parse_block(),
            Kind::Let => Ok(Statement::Let(self.parse_variable_declaration_statement()?)),
            _ => Err("Statement expected".to_string()),
        }
    }

    fn parse_block(&mut self) -> Result<Statement, String> {
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

        Ok(Statement::Block(open_brace, statements, close_brace))
    }

    fn parse_variable_declaration_statement(&mut self) -> Result<Let, String> {
        let let_token = self.next_token();

        let identifier_token = self.next_token();
        if identifier_token.kind != Kind::Identifier {
            return Err("Identifier expected".to_string());
        }

        let next = self.current_token();

        match next.kind {
            Kind::Equals => {
                let let_keyword = LetKeyword(let_token);
                let identifier = Identifier(identifier_token);

                let assignment_token = self.next_token();
                if assignment_token.kind != Kind::Equals {
                    return Err("Assignment operator expected".to_string());
                }

                let equals = AssignmentOperator(assignment_token);

                let expression = self.parse_expression(0)?;

                let semicolon_token = self.next_token();
                if semicolon_token.kind != Kind::Semicolon {
                    return Err("Semicolon expected".to_string());
                }

                let semicolon = Semicolon(semicolon_token);

                Ok(Let::UntypedWithValue(
                    let_keyword,
                    identifier,
                    equals,
                    expression,
                    semicolon,
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
