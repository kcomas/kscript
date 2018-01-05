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
    InvalidLocalSaveIndex(usize),
    InvalidLocalGetIndex(usize),
    // type errors
    NotAFunction(DataType),
    NotABool(DataType),
}
