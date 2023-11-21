use std::{
    fs::{self, File},
    io::Write,
};

use crate::lang::{generators::c_code_generator::CCodeGenerator, sematic::analyzer::Analyzer};

use super::syntax::{lexer::lexer::Lexer, parser::parser::Parser};

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
        let tokens = lexer.lex()?;

        let mut parser = Parser::from_tokens(tokens);
        let ast = parser.parse().map_err(|e| format!("{}", e))?;

        let mut analyzer = Analyzer::from_ast(ast.clone());
        let block_map = analyzer
            .analyze()
            .map_err(|semantic_error| format!("{:?}", semantic_error))?;

        let generator = CCodeGenerator::from_ast(ast, block_map);
        let code = generator.generate();

        println!("Compiled successfully!");
        println!("{}", code);

        let mut file = File::create("main.c").unwrap();
        file.write_all(code.as_bytes()).unwrap();

        Ok(())
    }
}
