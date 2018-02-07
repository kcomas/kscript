use super::data_type::DataType;

#[derive(Debug)]
pub enum Command {
    PushStack(DataType),
    Add,
    Sub,
    Call,
    LoadArgument(usize),
    Return,
    Halt(i32),
}
