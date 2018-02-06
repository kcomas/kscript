use super::data_type::DataType;

#[derive(Debug)]
pub enum Command {
    PushStack(DataType),
    Add,
    Sub,
    Halt(i32),
}
