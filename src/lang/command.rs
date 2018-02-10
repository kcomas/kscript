use super::memory::MemoryAddress;

#[derive(Debug)]
pub enum Command {
    PushStack(MemoryAddress),
    Add,
    Sub,
    Call,
    LoadArgument(usize),
    Return,
    Print,
    Halt(i32),
}
