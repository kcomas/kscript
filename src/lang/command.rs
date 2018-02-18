use super::address::MemoryAddress;

#[derive(Debug)]
pub enum Command {
    Push(MemoryAddress),
    Halt(i32),
}
