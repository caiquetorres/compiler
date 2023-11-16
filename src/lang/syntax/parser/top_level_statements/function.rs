use crate::lang::syntax::{
    parser::shared::{block::Block, identifier::Identifier},
    tree_display::TreeDisplay,
};

pub struct ParamDeclaration {
    pub identifier: Identifier,
    pub type_identifier: Identifier,
}

impl ParamDeclaration {
    pub fn new(identifier: Identifier, type_identifier: Identifier) -> Self {
        Self {
            identifier,
            type_identifier,
        }
    }
}

impl TreeDisplay for ParamDeclaration {
    fn display(&self, layer: usize) {
        println!(
            "{}ParamDeclaration ({}) ({})",
            " ".repeat(layer),
            self.identifier.name,
            self.type_identifier.name,
        );
    }
}

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
        for param in &self.params {
            param.display(layer);
        }
    }
}

pub struct Function {
    pub identifier: Identifier,
    pub params_declaration: ParamsDeclaration,
    pub type_identifier: Option<Identifier>,
    pub block: Block,
}

impl Function {
    pub fn new(
        identifier: Identifier,
        params_declaration: ParamsDeclaration,
        type_identifier: Option<Identifier>,
        block: Block,
    ) -> Self {
        Self {
            identifier,
            params_declaration,
            type_identifier,
            block,
        }
    }
}

impl TreeDisplay for Function {
    fn display(&self, layer: usize) {
        let id = self.identifier.name.clone();

        match self.type_identifier.as_ref() {
            Some(type_id) => {
                println!(
                    "{}FunctionDeclaration ({}) ({})",
                    " ".repeat(layer),
                    id,
                    type_id.name
                );
            }
            None => {
                println!("{}FunctionDeclaration ({})", " ".repeat(layer), id,);
            }
        };

        self.params_declaration.display(layer + 2);
        self.block.display(layer + 2);
    }
}
