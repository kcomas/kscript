use super::data_type::DataType;

#[derive(Debug)]
pub enum Command {
    PushStack(DataType),
    Add,
    Sub,
    Call,
    Return,
    Halt(i32),
}
