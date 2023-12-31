use super::{
    top_level_statements::top_level_statement::TopLevelStatement, tree_display::TreeDisplay,
};

#[derive(Clone, Debug)]
pub struct CompilationUnit {
    pub statements: Vec<TopLevelStatement>,
}

impl CompilationUnit {
    pub fn new(statements: Vec<TopLevelStatement>) -> Self {
        Self { statements }
    }

    #[allow(dead_code)]
    pub fn display(&self) {
        let statements = &self.statements;

        for statement in statements {
            statement.display(0);
        }
    }
}
