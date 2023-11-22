use std::{cell::RefCell, rc::Rc};

use crate::lang::syntax::parser::top_level_statements::function::Function;

use super::{
    block_analyzer::BlockAnalyzer, lang_type::LangType, scope::Scope,
    semantic_error::SemanticError, symbol::Symbol,
};

pub struct FunctionAnalyzer {
    pub(crate) diagnosis: Vec<SemanticError>,
}

impl FunctionAnalyzer {
    pub fn analyze(function: &Function, global_scope: Rc<RefCell<Scope>>) -> Self {
        let mut diagnosis: Vec<SemanticError> = vec![];

        let function_return_type: LangType;
        let function_name = function.identifier.name.clone();

        // Verify if the function was already declared or if some builtin identifier has the same name.
        if let Some(_) = global_scope.borrow().get(&function_name) {
            diagnosis.push(SemanticError::DuplicatedIdentifier);
        }

        // Verify the function return type.
        if let Some(type_identifier) = &function.type_identifier {
            let function_return_type_name = type_identifier.name.clone();

            // Verify if the function return type exists.
            if let None = global_scope.borrow().get(&function_return_type_name) {
                diagnosis.push(SemanticError::IdentifierNotFound);
            }

            function_return_type = LangType::from(function_return_type_name);
        } else {
            function_return_type = LangType::Void;
        }

        // Verify is the main function and if it has parameters.
        if function_name == "main" && function.params_declaration.params.len() != 0 {
            diagnosis.push(SemanticError::MainFunctionWithParameters);
        }

        let mut params_types: Vec<LangType> = vec![];

        for param_declaration in &function.params_declaration.params {
            let param_name = param_declaration.identifier.name.clone();
            let param_type_name = param_declaration.type_identifier.name.clone();

            // Verify if the parameter was already declared or if some builtin identifier has the same name.
            if let Some(_) = global_scope.borrow().get(&param_name) {
                diagnosis.push(SemanticError::DuplicatedIdentifier);
            }

            // Verify if the parameter type was already declared.
            if let None = global_scope.borrow().get(&param_type_name) {
                diagnosis.push(SemanticError::IdentifierNotFound);
            }

            params_types.push(LangType::from(param_type_name));
        }

        // Save the function in the global scope.
        global_scope.borrow_mut().insert(Symbol::Function {
            name: function_name,
            symbol_type: function_return_type.clone(),
            params: params_types,
        });

        // Creates the local function scope.
        let mut function_scope = Scope::new(global_scope, false, Some(function_return_type));

        for param_declaration in &function.params_declaration.params {
            let param_name = param_declaration.identifier.name.clone();
            let param_type_name = param_declaration.type_identifier.name.clone();

            // Verify if the parameter was already declared or if some builtin identifier has the same name.
            if let Some(_) = function_scope.get(&param_name) {
                diagnosis.push(SemanticError::DuplicatedIdentifier);
            }

            // Verify if the parameter type was already declared.
            if let None = function_scope.get(&param_type_name) {
                diagnosis.push(SemanticError::IdentifierNotFound);
            }

            function_scope.insert(Symbol::Parameter {
                name: param_name,
                symbol_type: LangType::from(param_type_name),
            })
        }

        let scope = Rc::new(RefCell::new(function_scope));
        let analyzer = BlockAnalyzer::analyze_within_scope(&function.block, scope);
        diagnosis.extend(analyzer.diagnosis);

        Self { diagnosis }
    }
}
