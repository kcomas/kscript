use super::data_type::DataType;

#[derive(Debug)]
pub enum RuntimeError {
    // run errors
    StackEmpty,
    CallsEmpty,
    CannotReturn,
    NoMoreCommands,
    InvalidNumberOfArguments,
    ArgumentsNotOnStack(usize),
    CannotLoadArgToStack(usize),
    // type errors
    NotAFunction(DataType),
    NotABool(DataType),
}
