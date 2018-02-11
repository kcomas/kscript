use super::memory::MemoryAddress;

#[derive(Debug)]
pub enum Command {
    PushStack(MemoryAddress),
    Equals,
    JumpIfFalse(usize),
    Add,
    Sub,
    Call,
    CallSelf,
    LoadArg(usize),
    Return,
    Print,
    Halt(i32),
}
