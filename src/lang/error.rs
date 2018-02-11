#[derive(Debug)]
pub enum RuntimeError {
    CannotLoadMemoryStackItem,
    CannotLoadMemoryFixedItem,
    CannotPopMemoryStack,
    CallStackEmpty,
    InvalidCommandIndex,
    VmStackEmpty,
    InvalidFunction,
    InvalidReturnStackLen,
    InvalidReturn,
    CannotLoadArgument,
    CannotCompareTypes,
    TargetNotABool,
}
