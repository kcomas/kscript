#[derive(Debug)]
pub enum RuntimeError {
    CannotPopStackEmpty,
    InvalidCommandIndex,
    CallsEmpty,
    InvalidFunction,
    InvalidNumberOfArguments,
    ArgumentsNotOnStack,
    CannotReturn,
    CannotLoadStackArgument,
    CannotCompareTypes,
    InvalidJumpBool,
    // Memory
    InvalidMemoryAccess,
    InvalidMemoryUpdate,
    CannotIncreaseRefCount,
    CannotDecreaseRefCount,
    CannotClearMemory,
    CannotLoadLocal,
    InvalidLocalSaveIndex,
    InvalidArgumentSaveIndex,
    InvalidIoWriteType,
    InvalidIoWriteFdIndex,
    InvalidIoAppendType,
    InvalidIoAppendFdIndex,
    // Update Errors
    UpdateTypeMismatch,
    InvalidType,
}

#[derive(Debug, Clone)]
pub enum ParserError {
    InvalidComment,
    InvalidVarNumber,
    InvalidReturn,
    InvalidGroup,
    InvalidBlock,
    InvalidAssign,
    InvalidEqualsGreaterLess,
    InvalidIoWriteIoAppendGreater,
}

#[derive(Debug)]
pub enum JoinError {
    InvalidIfStatement,
    InvalidSelfCallStatement,
    BlockShouldNotBeReached,
    InvalidFunctionVarSymbol,
    InvalidAssignVarSymbol,
    InvalidFunctionArgument,
    AssignShouldNotBeReached,
}

#[derive(Debug)]
pub enum ShuntError {
    FaileToPopOperatorStack,
}

#[derive(Debug)]
pub enum BuilderError {
    InvalidSingleAst,
    InvalidAstWithBody,
}
