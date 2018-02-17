#[derive(Debug)]
pub enum RuntimeError {
    CannotGetCountedMemoryItem,
    CannotGetFixedMemoryItem,
    CannotIncRef,
    CannotDecRef,
    CannotLoadCurrentFunction,
    CannotUpdateCurrentFunction,
    TargetIsNotBool,
    TargetIsNotAFunction,
    InvalidCommandIndex,
    StackEmpty,
    InvalidRetrunLength,
    CannotLoadArgument,
    CannotCompareTypes,
    CannotGetLastFunctionIndex,
}
