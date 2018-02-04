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
    Halt(i32),
}

impl Command {
    pub fn is_return(&self) -> bool {
        if let Command::Return = *self {
            return true;
        }
        false
    }

    pub fn is_halt(&self) -> bool {
        if let Command::Halt(_) = *self {
            return true;
        }
        false
    }
}
