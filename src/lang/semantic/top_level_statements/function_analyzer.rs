use crate::lang::position::Positioned;
use crate::lang::semantic::analyzer::Scopes;
use crate::lang::semantic::scope::Func;
use crate::lang::semantic::semantic_type::SemanticType;
use crate::lang::semantic::shared::type_analyzer::TypeAnalyzer;
use crate::lang::semantic::statements::block_analyzer::BlockAnalyzer;
use crate::lang::semantic::symbol::Symbol;
use crate::lang::semantic::{scope::Scope, semantic_error::SemanticError};
use crate::lang::syntax::top_level_statements::function::Function;

use std::{cell::RefCell, rc::Rc};

pub struct FunctionAnalyzer {
    pub(crate) diagnosis: Vec<SemanticError>,
}

impl FunctionAnalyzer {
    pub fn analyze_declaration(function: &Function, global_scope: Rc<RefCell<Scope>>) -> Self {
        let mut diagnosis: Vec<SemanticError> = vec![];

        let function_type: SemanticType;
        let function_name = function.identifier.name.clone();

        // Verify if the function was already declared or if some builtin identifier has the same name.
        if let Some(_) = global_scope.borrow().get(&function_name) {
            diagnosis.push(SemanticError::DuplicatedIdentifier {
                position: function.identifier.get_position(),
            });
        }

        let mut params_types: Vec<SemanticType> = vec![];

        for param_declaration in &function.params_declaration.params {
            let param_name = param_declaration.identifier.name.clone();

            // Verify if the parameter was already declared or if some builtin identifier has the same name.
            if let Some(_) = global_scope.borrow().get(&param_name) {
                diagnosis.push(SemanticError::DuplicatedIdentifier {
                    position: param_declaration.identifier.get_position(),
                });
            }

            let analyzer =
                TypeAnalyzer::analyze(&param_declaration.r#type, Rc::clone(&global_scope));
            diagnosis.extend(analyzer.diagnosis);
            let param_type = analyzer.result_type;

            params_types.push(param_type);
        }

        // Verify the function return type.
        if let Some(r#type) = &function.r#type {
            let analyzer = TypeAnalyzer::analyze(r#type, Rc::clone(&global_scope));
            diagnosis.extend(analyzer.diagnosis);
            function_type = SemanticType::Function(params_types, Box::new(analyzer.result_type));
        } else {
            function_type = SemanticType::Function(params_types, Box::new(SemanticType::Void));
        }

        if let SemanticType::Function(_, function_return_type) = &function_type {
            if matches!(function_return_type.as_ref(), SemanticType::Array(_, _)) {
                diagnosis.push(SemanticError::CannotReturnArray {
                    position: function.r#type.as_ref().unwrap().get_position(),
                })
            }
        }

        if let SemanticType::Function(_, function_return_type) = &function_type {
            if matches!(function_return_type.as_ref(), SemanticType::Function(_, _)) {
                diagnosis.push(SemanticError::CannotReturnFunction {
                    position: function.r#type.as_ref().unwrap().get_position(),
                })
            }
        }

        // Verify is the main function and if it has parameters.
        if function_name == "main" && function.params_declaration.params.len() != 0 {
            diagnosis.push(SemanticError::MainFunctionWithParameters {
                position: function.identifier.get_position(),
            });
        }

        if function_name == "main" {
            if let SemanticType::Function(_, function_return_type) = &function_type {
                if function_return_type.as_ref().clone() != SemanticType::Void {
                    diagnosis.push(SemanticError::MainFunctionWithReturn {
                        position: function.r#type.as_ref().unwrap().get_position(),
                    });
                }
            }
        }

        // Save the function in the global scope.
        global_scope.borrow_mut().insert(Symbol::Function {
            name: function_name.clone(),
            symbol_type: function_type,
        });

        Self { diagnosis }
    }

    pub fn analyze(
        function: &Function,
        global_scope: Rc<RefCell<Scope>>,
        scopes: &mut Scopes,
    ) -> Self {
        let mut diagnosis: Vec<SemanticError> = vec![];

        let function_name = function.identifier.name.clone();
        let mut function_return_type = SemanticType::Any;

        if let Some(symbol) = global_scope.borrow().get(&function_name) {
            if let Symbol::Function { symbol_type, .. } = &symbol {
                function_return_type = symbol_type.clone();
            }
        }

        // Creates the local function scope.
        let mut function_scope = Scope::new(
            Rc::clone(&global_scope),
            false,
            Some(Func {
                name: function_name,
                return_type: function_return_type,
            }),
        );

        for param_declaration in &function.params_declaration.params {
            let param_name = param_declaration.identifier.name.clone();

            // Verify if the parameter was already declared or if some builtin identifier has the same name.
            if let Some(_) = function_scope.get(&param_name) {
                diagnosis.push(SemanticError::DuplicatedIdentifier {
                    position: param_declaration.identifier.get_position(),
                });
            }

            let analyzer =
                TypeAnalyzer::analyze(&param_declaration.r#type, Rc::clone(&global_scope));
            diagnosis.extend(analyzer.diagnosis);
            let param_type = analyzer.result_type;

            function_scope.insert(Symbol::Parameter {
                name: param_name,
                symbol_type: param_type,
            })
        }

        let scope = Rc::new(RefCell::new(function_scope));
        let analyzer = BlockAnalyzer::analyze_within_scope(&function.block, scope, scopes);
        diagnosis.extend(analyzer.diagnosis);

        Self { diagnosis }
    }
}
