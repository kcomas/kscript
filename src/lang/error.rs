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
}
