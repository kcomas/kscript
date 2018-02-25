#[derive(Debug)]
pub enum RuntimeError {
    CannotLoadCountedMemory,
    CannotLoadFixedMemory,
    CannotIncRefCount,
    CannotDecRefCount,
    TargetIsNotABool,
    TargetIsNotAFunction,
    CannotLoadAddressIsData,
    CannotReloadData,
    CannotIncData,
    CannotLoadCurrentCall,
    CannotUpdateCurrentCall,
    InvalidCommandIndex,
    CannotPopStack,
    CannotReturnFromFunction,
    InvalidStackReturnLength,
    InvalidArgumentIndex,
    CannotCompareTypes,
    CannotAddTypes,
    CannotSubTypes,
}

#[derive(Debug, Clone)]
pub enum LexerError {
    InvalidTokenLength,
    EndOfFile,
    InvalidFloat,
    InvalidStringEscape,
}

#[derive(Debug)]
pub enum JoinerError {
    InvalidToken,
    TokenFnMismatch,
    NoMuliMatchFound,
    AstMultiMatchVecEmpty,
    InvalidVarForFnCall,
}
