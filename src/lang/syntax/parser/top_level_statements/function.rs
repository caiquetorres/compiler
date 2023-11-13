use crate::lang::syntax::{
    parser::shared::{block::Block, identifier::Identifier},
    tree_display::TreeDisplay,
};

pub struct ParamDeclaration(pub Identifier, pub Identifier);

impl TreeDisplay for ParamDeclaration {
    fn display(&self, layer: usize) {
        let id = self.0 .0.value.as_ref().unwrap();
        let type_id = self.1 .0.value.as_ref().unwrap();
        println!(
            "{}ParamDeclaration ({}) ({})",
            " ".repeat(layer),
            id,
            type_id
        );
    }
}

pub struct ParamsDeclaration(pub Vec<ParamDeclaration>);

impl TreeDisplay for ParamsDeclaration {
    fn display(&self, layer: usize) {
        for param in &self.0 {
            param.display(layer);
        }
    }
}

pub struct Function(
    pub Identifier,
    pub ParamsDeclaration,
    pub Option<Identifier>,
    pub Block,
);

impl TreeDisplay for Function {
    fn display(&self, layer: usize) {
        let id = self.0 .0.value.as_ref().unwrap();

        match self.2.as_ref() {
            Some(type_id) => {
                println!(
                    "{}FunctionDeclaration ({}) ({})",
                    " ".repeat(layer),
                    id,
                    type_id.0.value.as_ref().unwrap()
                );
            }
            None => {
                println!("{}FunctionDeclaration ({})", " ".repeat(layer), id,);
            }
        };

        self.1.display(layer + 2);
        self.3.display(layer + 2);
    }
}
