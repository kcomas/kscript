use super::data_type::DataType;

#[derive(Debug)]
pub enum Command {
    PushStack(DataType),
    Halt(i32),
}
