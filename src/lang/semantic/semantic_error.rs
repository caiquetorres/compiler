use super::lang_type::LangType;

#[derive(Debug)]
pub enum SemanticError {
    DuplicatedIdentifier,
    IdentifierNotFound,
    MainFunctionWithParameters, // fun main(a: i32) { ... }
    MainFunctionWithReturn,
    MissingTypeOrExpression,               // let x;
    UnaryOperatorOnlyApplicableToInteger,  // ~(1)
    UnaryOperatorOnlyApplicableToNumbers,  // +(true)
    UnaryOperatorOnlyApplicableToBooleans, // !(2)
    InvalidRangeOperands,                  // true..2
    IdentifierNotVariableConstOrParam,     // 2 + a
    IdentifierNotCallable,                 // a()
    InvalidNumberOfParameters,             // func(1, 2, 3)
    InvalidParameterType,                  // func(true)
    TypeMismatch,                          // a += true
    EqualityTypeMismatch,                  // 2 == true
    InvalidOperator,                       // 2 + true, 2.0 & 3.0, true >= true, 2 && 2
    ValueCannotBeReassigned,               // a = 2
    InvalidLeftOperand,                    // a += 2
    InvalidRightOperand,                   // a += true
    InvalidBreak,
    InvalidContinue,
    InvalidReturn,
    ExpectedType { expected: LangType, found: LangType },
}
