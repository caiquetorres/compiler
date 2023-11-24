use crate::lang::syntax::{
    parser::shared::{block::Block, identifier::Identifier, r#type::Type},
    tree_display::TreeDisplay,
};

#[derive(Clone, Debug)]
pub struct ParamDeclaration {
    pub identifier: Identifier,
    pub r#type: Type,
}

impl ParamDeclaration {
    pub fn new(identifier: Identifier, r#type: Type) -> Self {
        Self { identifier, r#type }
    }
}

impl TreeDisplay for ParamDeclaration {
    fn display(&self, _: usize) {
        // println!(
        //     "{}ParamDeclaration ({}) ({})",
        //     " ".repeat(layer),
        //     self.identifier.name,
        //     self.r#type.name,
        // );
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
        for param in &self.params {
            param.display(layer);
        }
    }
}

#[derive(Clone, Debug)]
pub struct Function {
    pub identifier: Identifier,
    pub params_declaration: ParamsDeclaration,
    pub r#type: Option<Type>,
    pub block: Block,
}

impl Function {
    pub fn new(
        identifier: Identifier,
        params_declaration: ParamsDeclaration,
        r#type: Option<Type>,
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
                print!("{}FunctionDeclaration ({}) ", " ".repeat(layer), id,);
                r#type.display(0);
            }
            None => {
                println!("{}FunctionDeclaration ({})", " ".repeat(layer), id,);
            }
        };

        self.params_declaration.display(layer + 2);
        self.block.display(layer + 2);
    }
}
