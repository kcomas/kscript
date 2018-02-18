use super::address::MemoryAddress;

#[derive(Debug)]
pub enum Command {
    Push(MemoryAddress),
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
