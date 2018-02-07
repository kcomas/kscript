#[derive(Debug)]
pub enum RuntimeError {
    NoMoreFrames,
    CannotReturnFromFrame,
    NoMoreCommands,
    StackEmpty,
    TargetNotAFunction,
}
