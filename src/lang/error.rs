#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
pub enum JoinError {
    InvalidIfStatement,
    InvalidSelfCallStatement,
    BlockShouldNotBeReached,
}

#[derive(Debug, Clone)]
pub enum ShuntError {

}
