use super::data_type::DataType;

#[derive(Debug)]
pub enum Command {
    PushStack(DataType),
    Equals,
    JumpIfFalse(usize),
    Add,
    Sub,
    Call,
    CallSelf,
    LoadArgument(usize),
    Return,
    Halt(i32),
}
