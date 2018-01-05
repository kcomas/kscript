use super::data_type::DataType;

#[derive(Debug)]
pub enum RuntimeError {
    // run errors
    StackEmpty,
    CallsEmpty,
    CannotReturn,
    NoMoreCommands,
    InvalidNumberOfArguments,
    // type errors
    NotAFunction(DataType),
    NotABool(DataType),
}
