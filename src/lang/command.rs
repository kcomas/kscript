use super::memory::MemoryAddress;

#[derive(Debug, Clone)]
pub enum Command {
    PushStack(MemoryAddress),
    LoadLocal(usize),
    SaveLocal(usize),
    Equals,
    JumpIfFalse(usize),
    Add,
    Sub,
    Call,
    CallSelf,
    LoadArgument(usize),
    SaveArgument(usize),
    Return,
    IoWrite,
    IoAppend,
    PrintDebug,
    Halt(i32),
}
