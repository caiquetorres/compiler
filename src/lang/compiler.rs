use std::{
    fs::{self, File},
    io::Write,
};

use super::{
    generators::c_code_generator::CCodeGenerator, lexer::lexer::Lexer,
    semantic::analyzer::Analyzer, syntax::parser::Parser,
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
        let tokens = lexer.lex()?;

        let mut parser = Parser::from_tokens(tokens);
        let ast = parser.parse().map_err(|e| format!("{}", e))?;

        let analyzer = Analyzer::analyze(&ast);

        if analyzer.diagnosis.len() > 0 {
            for error in &analyzer.diagnosis {
                println!("{}", error);
            }
        } else {
            let generator = CCodeGenerator::new(&ast, &analyzer.scopes);
            let code = generator.generate();

            let mut file = File::create("output.c").unwrap();
            file.write_all(code.as_bytes()).unwrap();
        }

        Ok(())
    }
}
