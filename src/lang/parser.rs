use super::{
    binary_expression_node::BinaryExpressionNode, kind::Kind, lexer::Lexer, node::Node,
    number_expression_node::NumberExpressionNode, operator_node::OperatorNode, token::Token,
    tree::Tree,
};

pub struct Parser {
    text: String,
}

impl Parser {
    pub fn new(text: &str) -> Self {
        Self {
            text: String::from(text),
        }
    }

    // REVIEW: Currently the parser is just parsing binary expressions such as "2 + 2"
    pub fn parse(&self) -> Result<Tree, String> {
        let binary_expression = self.parse_binary_expression()?;
        let tree = Tree::new(binary_expression);
        return Ok(tree);
    }

    fn parse_binary_expression(&self) -> Result<Box<dyn Node>, String> {
        let lexer = Lexer::new(&self.text);

        let tokens = lexer
            .filter(|token| token.kind != Kind::WhiteSpaceToken)
            .collect::<Vec<Token>>();

        // REVIEW: Should we improve this logic?
        let bad_token = tokens.iter().find(|token| token.kind == Kind::BadToken);

        if bad_token.is_some() {
            return Err(format!("Bad token: {}", bad_token.unwrap().text));
        }

        // REVIEW: Is there any possibility of the text being empty?

        let mut position: usize = 0;
        let mut token = tokens[position].clone();

        let root_token = token;

        position += 1;
        token = tokens
            .get(position)
            .ok_or("Error while parsing")?
            .to_owned();

        let mut binary_expression: Option<Box<BinaryExpressionNode>> = None;

        while token.kind == Kind::PlusToken || token.kind == Kind::MinusToken {
            let operator_token = tokens[position].clone();
            let operator_node = OperatorNode::new(operator_token);

            position += 1;
            token = tokens
                .get(position)
                .ok_or("Error while parsing")?
                .to_owned();

            let right_token = token.clone();
            let right = Box::new(NumberExpressionNode::new(right_token));

            if binary_expression.is_some() {
                binary_expression = Some(Box::new(BinaryExpressionNode::new(
                    binary_expression.unwrap(),
                    Box::new(operator_node),
                    right,
                )));
            } else {
                binary_expression = Some(Box::new(BinaryExpressionNode::new(
                    Box::new(NumberExpressionNode::new(root_token.clone())),
                    Box::new(operator_node),
                    right,
                )));
            }

            position += 1;
            match tokens.get(position) {
                Some(t) => token = t.to_owned(),
                None => break,
            }
        }

        if binary_expression.is_none() {
            Ok(Box::new(NumberExpressionNode::new(root_token)))
        } else {
            Ok(binary_expression.unwrap())
        }
    }
}
