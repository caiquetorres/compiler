use crate::lang::semantic::analyzer::Scopes;
use crate::lang::semantic::semantic_type::SemanticType;
use crate::lang::syntax::top_level_statements::function::Function;

use super::block_generator::BlockGenerator;
use super::c_code_generator2::CCode;

pub struct FunctionGenerator;

impl FunctionGenerator {
    pub fn generate(function: &Function, scopes: &Scopes, ccode: &mut CCode) {
        let name = function.identifier.name.clone();
        let is_main = name == "main";

        // Already validated in the semantic analyses
        let return_type = if is_main {
            SemanticType::I32
        } else {
            function.r#type.as_ref().map_or(SemanticType::Void, |id| {
                SemanticType::from_syntax(id.clone())
            })
        };

        let c_return_type = ccode.get_type(return_type);
        ccode.push(&format!("{} {}(", c_return_type, name));

        let mut params: Vec<String> = vec![];

        for param in &function.params_declaration.params {
            let param_type = SemanticType::from_syntax(param.r#type.clone());

            let c_param_type = ccode.get_type(param_type);
            params.push(format!("{} {}", c_param_type, param.identifier.name));
        }

        ccode.push(&params.join(","));

        ccode.push(")");

        BlockGenerator::generate(&function.block, scopes, ccode);

        if is_main {
            ccode.pop();
            ccode.push("return 0;}");
        }
    }

    pub fn generate_prototype(function: &Function, ccode: &mut CCode) {
        let name = function.identifier.name.clone();
        let is_main = name == "main";

        if is_main {
            return;
        }

        // Already validated in the semantic analyses
        let return_type = function
            .r#type
            .as_ref()
            .map_or(SemanticType::Void, |r#type| {
                SemanticType::from_syntax(r#type.clone())
            });

        let c_return_type = ccode.get_type(return_type);
        ccode.push(&format!("{} {}(", c_return_type, name));

        let mut params: Vec<String> = vec![];

        for param in &function.params_declaration.params {
            let param_type = SemanticType::from_syntax(param.r#type.clone());

            let c_param_type = ccode.get_type(param_type);
            params.push(c_param_type);
        }

        ccode.push(&params.join(","));

        ccode.push(");");
    }
}
