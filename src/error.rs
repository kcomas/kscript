use super::data_type::DataType;

#[derive(Debug)]
pub enum RuntimeError {
    StackEmpty,
    CallsEmpty,
    CannotReturn,
    NoMoreCommands,
    NotAFunction(DataType),
}
