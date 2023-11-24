use crate::lang::syntax::{
    parser::shared::{block::Block, identifier::Identifier, syntax_type::SyntaxType},
    tree_display::TreeDisplay,
};

#[derive(Clone, Debug)]
pub struct ParamDeclaration {
    pub identifier: Identifier,
    pub r#type: SyntaxType,
}

impl ParamDeclaration {
    pub fn new(identifier: Identifier, r#type: SyntaxType) -> Self {
        Self { identifier, r#type }
    }
}

impl TreeDisplay for ParamDeclaration {
    fn display(&self, layer: usize) {
        println!(
            "{}ParamDeclaration ({}: {})",
            "  ".repeat(layer),
            self.identifier.name,
            self.r#type.to_string()
        );
    }
}

#[derive(Clone, Debug)]
pub struct ParamsDeclaration {
    pub params: Vec<ParamDeclaration>,
}

impl ParamsDeclaration {
    pub fn new(params: Vec<ParamDeclaration>) -> Self {
        Self { params }
    }
}

impl TreeDisplay for ParamsDeclaration {
    fn display(&self, layer: usize) {
        if self.params.len() == 0 {
            return;
        }

        println!("{}ParamsDeclaration", "  ".repeat(layer));

        for param in &self.params {
            param.display(layer + 1);
        }
    }
}

#[derive(Clone, Debug)]
pub struct Function {
    pub identifier: Identifier,
    pub params_declaration: ParamsDeclaration,
    pub r#type: Option<SyntaxType>,
    pub block: Block,
}

impl Function {
    pub fn new(
        identifier: Identifier,
        params_declaration: ParamsDeclaration,
        r#type: Option<SyntaxType>,
        block: Block,
    ) -> Self {
        Self {
            identifier,
            params_declaration,
            r#type,
            block,
        }
    }
}

impl TreeDisplay for Function {
    fn display(&self, layer: usize) {
        let id = self.identifier.name.clone();

        match self.r#type.as_ref() {
            Some(r#type) => {
                println!(
                    "{}FunctionDeclaration ({}: {})",
                    "  ".repeat(layer),
                    id,
                    r#type.to_string()
                );
            }
            None => {
                println!("{}FunctionDeclaration ({})", "  ".repeat(layer), id,);
            }
        };

        self.params_declaration.display(layer + 1);
        self.block.display(layer + 1);
    }
}

// ├
// ─
// │
// └
