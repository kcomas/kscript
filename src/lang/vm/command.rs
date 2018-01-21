use super::data_type::DataType;

#[derive(Debug)]
pub enum Command {
    PushStack(DataType),
    Call,
    Return,
    Halt(i32),
}
