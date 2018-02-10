use super::memory::MemoryAddress;

#[derive(Debug)]
pub enum Command {
    PushStack(MemoryAddress),
    Halt(i32),
}
