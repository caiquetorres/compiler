use super::lang_type::LangType;

#[derive(Debug)]
pub enum SemanticError {
    DuplicatedIdentifier,
    IdentifierNotFound,
    MainFunctionWithParameters,       // fun main(a: i32) { ... }
    MissingTypeOrExpression,          // let x;
    NotOnlyInIntegers,                // ~(1)
    OperatorOnlyInNumbers,            // +(true)
    OperatorOnlyInBooleans,           // !(2)
    InvalidRangeOperands,             // true..2
    IdentifierNotAVariableConstParam, // 2 + a
    IdentifierNotCallable,            // a()
    InvalidParamsAmount,              // func(1, 2, 3)
    InvalidParam,                     // func(true)
    MismatchedEqualityTypes,          // 2 == true
    InvalidOperator,                  // 2 + true, 2.0 & 3.0, true >= true, 2 && 2
    ExpectedType { expected: LangType, found: LangType },
}
