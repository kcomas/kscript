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
}
