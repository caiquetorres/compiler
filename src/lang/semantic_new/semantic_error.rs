#[derive(Debug)]
pub enum SemanticError {
    DuplicatedIdentifier,
    IdentifierNotFound,
    MainFunctionWithParameters,
}
