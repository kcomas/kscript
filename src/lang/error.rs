#[derive(Debug)]
pub enum RuntimeError {
    InvalidMemoryAccess,
    InvalidRefInc,
    InvalidRefDec,
    InvalidUpdateAddress,
    CannotUpdateStaticMemory,
    CannotUpdateBool,
    CannotUpdateInteger,
    CannotUpdateFloat,
    CannotUpdateFunction,
    CallStackEmpty,
    InvalidCommandIndex,
    StackEmpty,
}
