use std::fs;

use super::{
    sematic::analyzer::Analyzer,
    syntax::{lexer::lexer::Lexer, parser::parser::Parser},
};

pub struct Compiler {
    code: String,
}

impl Compiler {
    pub fn from_file(file_path: &str) -> Result<Self, String> {
        let file_content = fs::read_to_string(file_path).map_err(|e| e.to_string())?;

        Ok(Compiler::from_code(&file_content))
    }

    pub fn from_code(code: &str) -> Self {
        Self {
            code: String::from(code),
        }
    }

    pub fn compile(&self) -> Result<(), String> {
        let mut lexer = Lexer::new(&self.code);
        let tokens = lexer.lex();

        let mut parser = Parser::from_tokens(tokens);
        let ast = parser.parse()?;

        let mut analyzer = Analyzer::from_ast(ast);
        analyzer.analyze()?;

        println!("Compiled successfully!");

        Ok(())
    }
}
