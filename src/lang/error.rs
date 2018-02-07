#[derive(Debug)]
pub enum RuntimeError {
    NoMoreFrames,
    CannotReturnFromFrame,
    NoMoreCommands,
    StackEmpty,
    TargetNotAFunction,
    TargetNotABool,
    InvalidFunctionStackLength,
    CannotLoadStackPositionToFront,
    CannotCompareDifferentTypes,
}
