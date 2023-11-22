use std::{cell::RefCell, collections::HashMap, rc::Rc};

use uuid::Uuid;

use crate::lang::syntax::parser::{
    compilation_unit::CompilationUnit, top_level_statements::top_level_statement::TopLevelStatement,
};

use super::{
    function_analyzer::FunctionAnalyzer, lang_type::LangType, scope::Scope,
    semantic_error::SemanticError, symbol::Symbol,
};

pub struct Analyzer {
    pub scopes: HashMap<Uuid, Rc<RefCell<Scope>>>,
    pub diagnosis: Vec<SemanticError>,
}

impl Analyzer {
    pub fn analyze(ast: &CompilationUnit) -> Self {
        let mut diagnosis: Vec<SemanticError> = vec![];
        let global_scope = Rc::new(RefCell::new(Scope::global()));

        let default_types = [
            LangType::U8,
            LangType::I8,
            LangType::U16,
            LangType::I16,
            LangType::U32,
            LangType::I32,
            LangType::U64,
            LangType::I64,
            LangType::F32,
            LangType::F64,
            LangType::Void,
            LangType::Bool,
            LangType::Char,
            LangType::String,
        ];

        for default_type in default_types {
            global_scope.borrow_mut().insert(Symbol::Type {
                name: default_type.to_string(),
            })
        }

        for statement in &ast.statements {
            match statement {
                TopLevelStatement::Function(function) => {
                    let analyzer = FunctionAnalyzer::analyze(function, Rc::clone(&global_scope));

                    diagnosis.extend(analyzer.diagnosis);
                }
            }
        }

        Self {
            scopes: HashMap::new(),
            diagnosis,
        }
    }
}
