use super::scope::Scope;
use super::semantic_error::SemanticError;
use super::semantic_type::SemanticType;
use super::symbol::Symbol;
use super::top_level_statements::function_analyzer::FunctionAnalyzer;

use crate::lang::syntax::parser::compilation_unit::CompilationUnit;
use crate::lang::syntax::parser::top_level_statements::top_level_statement::TopLevelStatement;

use std::{cell::RefCell, collections::HashMap, rc::Rc};
use uuid::Uuid;

pub type Scopes = HashMap<Uuid, Rc<RefCell<Scope>>>;

pub struct Analyzer {
    pub scopes: Scopes,
    pub diagnosis: Vec<SemanticError>,
}

impl Analyzer {
    pub fn analyze(ast: &CompilationUnit) -> Self {
        let mut diagnosis: Vec<SemanticError> = vec![];
        let mut scopes = Scopes::new();

        let global_scope = Rc::new(RefCell::new(Scope::global()));

        let default_types = [
            SemanticType::U8,
            SemanticType::I8,
            SemanticType::U16,
            SemanticType::I16,
            SemanticType::U32,
            SemanticType::I32,
            SemanticType::U64,
            SemanticType::I64,
            SemanticType::F32,
            SemanticType::F64,
            SemanticType::Void,
            SemanticType::Bool,
            SemanticType::Char,
            SemanticType::String,
        ];

        for default_type in default_types {
            global_scope.borrow_mut().insert(Symbol::Type {
                name: default_type.to_string(),
            })
        }

        for statement in &ast.statements {
            match statement {
                TopLevelStatement::Function(function) => {
                    let analyzer =
                        FunctionAnalyzer::analyze_declaration(function, Rc::clone(&global_scope));

                    diagnosis.extend(analyzer.diagnosis);
                }
            }
        }

        for statement in &ast.statements {
            match statement {
                TopLevelStatement::Function(function) => {
                    let analyzer =
                        FunctionAnalyzer::analyze(function, Rc::clone(&global_scope), &mut scopes);

                    diagnosis.extend(analyzer.diagnosis);
                }
            }
        }

        Self { scopes, diagnosis }
    }
}
